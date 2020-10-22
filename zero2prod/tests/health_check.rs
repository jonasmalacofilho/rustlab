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
    let server = setup_server().unwrap();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/health_check", server))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
