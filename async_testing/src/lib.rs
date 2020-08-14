pub async fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_tokio_block_on() {
        use tokio_test::block_on;

        assert_eq!(block_on(greeting("tokio_test")), "Hello, tokio_test!");
    }

    #[tokio::test]
    async fn with_tokio_test_attribute() {
        // requires tokio with features "macros" and "rt-core" | "rt-threaded"

        assert_eq!(greeting("tokio::test").await, "Hello, tokio::test!");
    }
}
