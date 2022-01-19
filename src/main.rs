use std::net::TcpListener;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind addr");
    let addr = zero2prod_axum::run(listener).unwrap();
    println!("{}", addr);
}
