mod server_http;
mod server_tcp;
use std::io::stdin;

use server_http::server_http::init_server_http_new;
use server_tcp::server_tcp::init_server_tcp;

#[tokio::main]
async fn main() {
    menu().await
}

async fn menu() {
    loop {
        let opt = get_opt();
        match opt.trim() {
            "1" => init_server_http_new().await,
            "2" => init_server_tcp().await,
            "exit" => break,
            _ => println!("Invalid option"),
        }
    }
    println!("Bye!");
}

fn get_opt() -> String {
    let mut opt: String = String::new();
    println!("1 - Init http server");
    println!("2 - Init Tcp server");
    stdin().read_line(&mut opt).expect("Failed read line");
    opt
}
