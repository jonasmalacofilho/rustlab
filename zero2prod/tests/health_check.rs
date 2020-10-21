use std::net::{SocketAddr, TcpListener};

use zero2prod;

fn setup_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    // actix_rt::spawn requires a future with Output = ()
    async fn run_server(listener: TcpListener) {
        zero2prod::run(listener).unwrap().await.unwrap()
    }

    actix_rt::spawn(run_server(listener));

    addr
}

#[actix_rt::test]
async fn returns_200_with_empty_body() {
    let server = setup_server();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/health_check", server))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
