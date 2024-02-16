use proc_macro;
// declaritive macros with macro_rules!
// three kinds of procedural macros
// 1) custom #[derive] that specify code added used on structs and enums
// 2) attribute like macros that define custom attributes usable on any item

// 3) function like macros that look like function calls but operate on tokens specified as their arguments
//
// a macro is a way of writing code that writes other code
// can take a variable number of parameters


// use $ to declare a variable in the macro system that will contain the rust code matching the pattern
// $x:expr matches any rust expression and gives it the name $x
// , indicates that a comman can optionally appear, * means it matches zero or more


#[macro_export] // indicates that the macro should be made available whenever the crate in which the macro is defined is brough into scope
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


// procedural macro acts more like a function
// custom derive, attribute like, and function like

// source code the macro is operating on makes up the input TokenStream and the code the macro
// produces is the output token stream

// writing a derive macro
pub trait HelloMacro {
    fn hello_macro();
}


