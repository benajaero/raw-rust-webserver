// Credit: Ben Ajaero

use raw::{logger, request_id, static_files, App, Response, Text};

#[tokio::main]
async fn main() {
    let mut app = App::new();
    app.add_layer(logger());
    app.add_layer(request_id());
    app.add_layer(static_files("public"));

    app.get("/", |_req| async { Response::from(Text::new("Hello from Raw")) });
    app.get("/health", |_req| async { Response::from(Text::new("ok")) });

    app.listen("127.0.0.1:3000").await.unwrap();
}
