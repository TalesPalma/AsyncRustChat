use std::net::SocketAddr;

use askama::Template;
use warp::Filter;

#[derive(Template)]
#[template(path = "index.html")]
struct HttpResponseHtmlServer {
    body: String,
}

pub async fn init_server_http_new() {
    tokio::spawn(async move {
        let addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        let route = warp::path::end().map(move || response());
        warp::serve(route).run(addr).await;
    });
    println!("Serving on");
}

fn response() -> impl warp::Reply {
    let template = HttpResponseHtmlServer {
        body: "teste".to_string(),
    }
    .render()
    .unwrap();

    warp::reply::html(template)
}
