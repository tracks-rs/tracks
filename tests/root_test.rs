mod test;
use test::run_testing_environment;
use reqwest; // HTTP Client

#[tokio::test]
async fn test_root() {
    let _app = run_testing_environment();

    // Make HTTP requests to your app using `reqwest`
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:3000/")
                        .send()
                        .await
                        .expect("Failed to send request");

    // Assert on the response
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let body = response.text().await.expect("Failed to read response text");
    assert_eq!(body, "Hello, World!");
    assert_ne!(body, "Not Hello World!");
    
}
