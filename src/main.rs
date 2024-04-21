use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    println!("port = {}",listener.local_addr().unwrap().port());
    zero2prod::run(listener)?.await
}
