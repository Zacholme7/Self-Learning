
fn main() {
    // macros refers to a family of features in rust
    // cuseom #[derive] macros that specify code added with derive attribute
    // attribute like macros that define cusom attributes usable on any item
    // function like macros that look like function calls but operate on the tokens specified as their argument

    // macros are a way of writing code that writes other code, metaprogramming
    // macros can take a variable number of parameters
    // must define macros or bring them into scope before you call them in a file

    // delclarative macros/macros by example
    // allow you to write something similar to match expressio
    // use macro_rules! construct to define macros
    // 1

    // procedurla marcros for generating code from attributes
    // accept some code as input, operate on the code, produce some code as output
    // definition must reside in their own create with special crate types
    // 2
}


// 1, vec macro
#[macro_export] // indicates macro should be made available whenever the crate in which the macro is defined is brought into scope
macro_rules! vec { // start with macro_rules! and name of macro
    ( $( $x:expr ),* ) => { // matching a pattern
        {
            let mut temp_vec = Vec::new();
            $( // use dollar sign to declare a variable 
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 2
use proc_macro
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    // ...
}