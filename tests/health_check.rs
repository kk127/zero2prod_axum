use reqwest::header::CONTENT_TYPE;
use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    zero2prod_axum::run(listener).unwrap()
}

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

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    let server_addr = spawn_app().await;
    let client = reqwest::Client::new();
    let dest_addr = format!("http://{server_addr}/subscribe");

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(dest_addr)
        .body(body)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await
        .unwrap();

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_return_a_400_when_data_is_missing() {
    let server_addr = spawn_app().await;
    let client = reqwest::Client::new();
    let dest_addr = format!("http://{server_addr}/subscribe");

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&dest_addr)
            .body(invalid_body)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The api doesn't fall with 400 Bad Request when the payload is {}",
            error_message
        )
    }
}
