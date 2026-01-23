// Credit: Ben Ajaero

use std::net::TcpListener;

use raw::{App, Response, RoutePolicy, StatusCode, Text};
use tokio::sync::oneshot;

#[tokio::test]
async fn serves_basic_route() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
    let addr = listener.local_addr().expect("local addr");

    let mut app = App::new();
    app.get("/", |_req| async { Response::from(Text::new("hello")) });

    let server = tokio::spawn(app.serve(listener));

    let client = hyper::Client::new();
    let uri = format!("http://{}/", addr).parse().expect("uri");
    let res = client.get(uri).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(res).await.expect("body bytes");
    assert_eq!(body, "hello");

    server.abort();
}

#[tokio::test]
async fn rejects_when_over_capacity() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
    let addr = listener.local_addr().expect("local addr");

    let mut app = App::new();
    let policy = RoutePolicy {
        max_in_flight: Some(1),
        cost: 1,
    };
    let (tx, rx) = oneshot::channel::<()>();
    let rx = std::sync::Arc::new(tokio::sync::Mutex::new(Some(rx)));
    app.get_with("/slow", policy, move |_req| {
        let rx = std::sync::Arc::clone(&rx);
        async move {
            let mut guard = rx.lock().await;
            if let Some(rx) = guard.take() {
                let _ = rx.await;
            }
            Response::from(Text::new("done"))
        }
    });

    let server = tokio::spawn(app.serve(listener));

    let client = hyper::Client::new();
    let uri: hyper::Uri = format!("http://{}/slow", addr).parse().expect("uri");
    let first = tokio::spawn(client.get(uri.clone()));
    tokio::task::yield_now().await;
    let second = client.get(uri).await.expect("second response");
    assert_eq!(second.status(), StatusCode::SERVICE_UNAVAILABLE);

    let _ = tx.send(());
    let _ = first.await;
    server.abort();
}
