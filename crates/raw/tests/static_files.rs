// Credit: Ben Ajaero

use std::fs;
use std::net::TcpListener;
use std::time::{SystemTime, UNIX_EPOCH};

use raw::{static_files, App, Response, StatusCode, Text};

#[tokio::test]
async fn serves_static_file_before_routes() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("timestamp")
        .as_millis();
    let root = std::env::temp_dir().join(format!("raw-static-{}", timestamp));
    fs::create_dir_all(&root).expect("create dir");
    fs::write(root.join("index.html"), "static-content").expect("write file");

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
    let addr = listener.local_addr().expect("local addr");

    let mut app = App::new();
    app.add_layer(static_files(&root));
    app.get("/", |_req| async { Response::from(Text::new("dynamic")) });

    let server = tokio::spawn(app.serve(listener));

    let client = hyper::Client::new();
    let uri = format!("http://{}/", addr).parse().expect("uri");
    let res = client.get(uri).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(res).await.expect("body bytes");
    assert_eq!(body, "static-content");

    server.abort();
    let _ = fs::remove_dir_all(&root);
}
