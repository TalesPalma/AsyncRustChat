use std::{env, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::{broadcast, Mutex},
};

pub async fn init_server_tcp(rx: Arc<Mutex<broadcast::Receiver<String>>>) {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("Listening on:{}", addr);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let rx = rx.clone();
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("Failed to read data from socket");

                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("Failed to write data to socket");

                // Recebe mensagem
                let mut rx = rx.lock().await;
                match rx.recv().await {
                    Ok(msg) => {
                        println!("Conteudo da mensagem {}", msg)
                    }
                    Err(_) => eprintln!("Error on receiving message"),
                }
            }
        });
    }
}
