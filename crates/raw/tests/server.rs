// Credit: Ben Ajaero

use std::net::TcpListener;

use raw::{App, Response, StatusCode, Text};

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
