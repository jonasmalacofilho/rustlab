use reqwest::multipart::Form;
use reqwest::{Client, StatusCode};
use std::net::{SocketAddr, TcpListener};

use zero2prod;

fn setup_server() -> Result<SocketAddr, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    tokio::spawn(zero2prod::run(listener)?);

    Ok(addr)
}

#[actix_rt::test]
async fn returns_200_with_empty_body() {
    let server = setup_server().expect("test setup failed");

    let client = Client::new();

    let response = client
        .get(&format!("http://{}/health_check", server))
        .send()
        .await
        .expect("request failed");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[actix_rt::test]
async fn subscribes_valid_name_and_email() {
    let server = setup_server().expect("test setup failed");

    let client = Client::new();

    let form = [("name", "Aria"), ("email", "to@aria.me")];

    let response = client
        .post(&format!("http://{}/subscribe", server))
        .form(&form)
        .send()
        .await
        .expect("request failed");

    assert!(response.status().is_success());
}

#[actix_rt::test]
async fn refuses_missing_name_or_email() {
    let server = setup_server().expect("test setup failed");

    let client = Client::new();

    // TODO try 3 different test functions that all call the same helper
    let test_cases = vec![
        ([("name", "Aria")], "is missing email"),
        ([("email", "to@aria.me")], "is missing name"),
        ([("_", "__")], "is missing both name and email"),
    ];

    for (form, desc) in test_cases {
        let response = client
            .post(&format!("http://{}/subscribe", server))
            .form(&form)
            .send()
            .await
            .expect("request failed");

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "failed to return 400 error when form {}",
            desc
        );
    }
}
