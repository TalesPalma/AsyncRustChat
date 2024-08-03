use askama::Template;
use std::{
    io::{self, BufRead, BufReader},
    net::SocketAddr,
    sync::Arc,
};
use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
};
use warp::Filter;

#[derive(Template)]
#[template(path = "index.html")]
struct HttpResponseHtmlServer {
    body: String,
}

fn render_page(message: String) -> impl warp::Reply {
    let template = HttpResponseHtmlServer {
        body: format!("Message: {}", message),
    };
    warp::reply::html(template.render().unwrap())
}

#[tokio::main]
async fn main() {
    let (tx, rx) = broadcast::channel::<String>(64);
    let rx = Arc::new(Mutex::new(rx));
    let rx_http = Arc::clone(&rx);
    tokio::join!(init_server_tcp(tx.clone(), rx), init_server_http(rx_http));
}

async fn init_server_http(rx: Arc<Mutex<broadcast::Receiver<String>>>) {
    let http_server = tokio::spawn(async move {
        let addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        let routes = warp::any().map(move || Arc::clone(&rx)).and_then(
            |rx: Arc<Mutex<broadcast::Receiver<String>>>| async move {
                loop {
                    let mut rx = rx.lock().await;
                    let message = rx.recv().await.unwrap_or_else(|_| "Nada ainda".to_string());
                    return Ok::<_, warp::Rejection>(render_page(message));
                }
            },
        );

        warp::serve(routes).run(addr).await;
    });
    println!("Server running on http://127.0.0.1:8081");
    http_server.await.unwrap();
}

async fn init_server_tcp(
    tx: broadcast::Sender<String>,
    rx: Arc<Mutex<broadcast::Receiver<String>>>,
) {
    let port = String::from("8080");
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // Clonando o transmissor
    let server = Arc::new(Mutex::new(Server::new(port, listener, rx)));
    let tx_clone = tx.clone();

    let tcp_server_task = tokio::spawn(async move {
        tcp_server_task(server).await;
    });

    let input_task = tokio::spawn(async move {
        input_task(tx_clone).await;
    });

    tcp_server_task.await.unwrap();
    input_task.await.unwrap();
}

async fn tcp_server_task(server: Arc<Mutex<Server>>) {
    let mut server = server.lock().await;
    if let Ok(_) = server.run().await {
        println!(
            "Server running on port {} finished sucessfully",
            server.port
        );
    }
}

async fn input_task(tx_clone: broadcast::Sender<String>) {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut reader_line = reader.lines();

    loop {
        if let Some(line) = reader_line.next() {
            let input = line.unwrap();
            if input.eq("stop") {
                println!("Parando servidor");
                tx_clone.send("stop".to_string()).unwrap();
            }
            tx_clone.send(input).unwrap();
        }
    }
}

struct Server {
    port: String,
    server: TcpListener,
    rx: Arc<Mutex<broadcast::Receiver<String>>>,
}

impl Server {
    fn new(port: String, server: TcpListener, rx: Arc<Mutex<broadcast::Receiver<String>>>) -> Self {
        Server { port, server, rx }
    }
    async fn run(&mut self) -> io::Result<()> {
        loop {
            tokio::select! {
                teste = self.server.accept()=>{
                    println!("Cliente connectado {}",teste.unwrap().1);

                }
            }

            if let Ok(signal) = self.rx.lock().await.recv().await {
                if signal == "stop" {
                    self.stop();
                    break;
                }
            }
        }
        Ok(())
    }

    fn stop(&self) {
        println!("Stopping server");
    }
}
