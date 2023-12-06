use Zero_To_Prod::run;

// just takes our main asynchronous body and writes the necessary boilerplate to make it run on top of tokios runtime
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}

