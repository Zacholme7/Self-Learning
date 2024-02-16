//! tests/health_check.rs
use std::net::TcpListener;

// Launch our application in the background 
// this is the only piece that will really depend on the application
fn spawn_app() -> String {
    // use tcplistener to bind to a random port
    // port 0 will make os search for port
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    // get the port number that os assigned to use
    let port = listener.local_addr().unwrap().port();

    // run the server
    let server = Zero_To_Prod::run(listener).expect("failed to bind address");

    // launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    // although we have no sue for it so we have a non binding let
    let _ = tokio::spawn(server);

    // return the application address to the caller
    format!("http://127.0.0.1:{}", port)
}

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
// spins up a new runtime at the beginning of each test case and shuts it down at the end
#[tokio::test]
async fn health_check_works() {
    // start the server in the background
    let address = spawn_app();

    // create a client so we can send request to server
    let client = reqwest::Client::new();

    // send request to server to our health_check endpoint
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // confim the response
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// test that when we subscribe we get a 200 response
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // spawn the server
    let app_address = spawn_app();
    // make our client that will send the requests
    let client = reqwest::Client:: new();

    // the body of the request that we want to send
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // send the request and get a reponse
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // assert our response
    assert_eq!(200, response.status().as_u16());
}

// make sure we get a 400 response when the data is invalid
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // spawn the server
    let app_address = spawn_app();
    // make our client that will send the requests
    let client = reqwest::Client:: new();

    // the cases that we want to test
    let test_cases = vec! [
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    // run all of our test cases
    for (invalid_body, error_message) in test_cases {
        // send the request and get a reponse
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // assert our response
        assert_eq!(400, response.status().as_u16(), "The API did not fail with 400 Bad Request when the payload was {}", error_message);
    }
}






























