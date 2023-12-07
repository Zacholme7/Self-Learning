use Zero_To_Prod::run;
use std::net::TcpListener;

// just takes our main asynchronous body and writes the necessary boilerplate to make it run on top of tokios runtime
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");

    // get the port number that os assigned to use
    let server = run(listener).expect("Failed to bind address");

    Ok(())
}

