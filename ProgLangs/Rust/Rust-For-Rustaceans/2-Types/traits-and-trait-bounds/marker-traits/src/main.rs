// MARKET TRAITS notes
// ------------------
// - compile time guarantees about certain properties/behaviors of types
// - flags/markers for compiler
// - marker traits are traits with no method or assocaited items
//  - used to indicate that something about that type implements them
//  - Send, Sync, Copy, Sized, Unpin, ...
// - also known as auto traits (except for copy) since compiler auto implements them
// - allow you to write bounds that capture semantic requirments not directly expressed in code
// - marker types: hold no data and have no methods
//  - mark a type as being in a particular state

// consider api that manages ssh connections, can be authenticated or unauthenticated
struct Unauthenticated;
struct Authenticated;

struct SshConnection<State> {
    state: std::marker::PhantomData<State>,
}

impl SshConnection<Unauthenticated> {
    // init new unauthenticated connection
    fn new() -> Self {
        SshConnection {
            state: std::marker::PhantomData,
        }
    }

    fn authenticate(self) -> SshConnection<Authenticated> {
        println!("Connection authenticated");
        SshConnection {
            state: std::marker::PhantomData,
        }
    }
}

impl SshConnection<Authenticated> {
    fn execute_command(&self, command: &str) {
        println!("Executing command: {}", command);
        // Implementation of command execution on an authenticated connection
    }
}

fn main() {
    let conn = SshConnection::<Unauthenticated>::new();
    let authenticated_conn = conn.authenticate();

    authenticated_conn.execute_command("ls -la");
}
