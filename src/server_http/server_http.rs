use std::{net::SocketAddr, sync::Arc};

use askama::Template;
use tokio::sync::{broadcast, Mutex};
use warp::Filter;

#[derive(Template)]
#[template(path = "index.html")]
struct HttpResponseHtmlServer {
    body: String,
}

pub async fn init_server_http_new(tx: Arc<Mutex<broadcast::Sender<String>>>) {
    tokio::spawn(async move {
        let addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        let route = warp::path::end().map(move || response());
        warp::serve(route).run(addr).await;
    });
    enviar_msg_test(tx).await;
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

async fn enviar_msg_test(tx: Arc<Mutex<broadcast::Sender<String>>>) {
    let tx = tx.lock().await;
    if let Err(e) = tx.send("Hello".to_string()) {
        eprintln!("Error on sending message: {}", e)
    }
}
