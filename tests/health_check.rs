use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let server_addr = spawn_app().await;

    let client = reqwest::Client::new();

    let dest_addr = format!("http://{server_addr}/health_check");

    let response = client
        .get(dest_addr)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    zero2prod_axum::run(listener).unwrap()
}
