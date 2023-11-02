use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // can respond to most errors

    // will return a result
    let greeting_file_result = File::open("hello.txt");

    // check to see if the file opened okay
    // if it did not open okay, see what error and respond
    let greeting_file = match greeting_file_result {
        Ok(file) => file
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file"),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // shortcuts for panic on error
    // unwrap: if Result is OK, that will return what is inside the ok, if ERr, it will call panic
    let greeting_file = File::open("hello.txt").unwrap();

    // expect acts the same way but lets us add error message
    let greeting_file = File::open("hello.txt").expect("could not find hello.txt");

    // ? operator can only be used in fucntions whose return type is compatible with the value the ? is used on
    // ? can also be used on options to return some or none
    
}

// can propogate the error to the caller
fn read_username_from_file() -> Result<String, io::Error> {
    // ? is placed after a result value
    // if result is OK, then value inside ok will get returned
    // if vlaue is err, err will be returned from the whole function
    // err values that have ? called on them will go through the from function
    // err type received is convered into the error type defined in the return type of the current function
    let username_file_result = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
