//! tests/health_check.rs

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

// Launch our application in the background 
// this is the only piece that will really depend on the application
async fn spawn_app() -> Result<(), std::io::Error> {
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

