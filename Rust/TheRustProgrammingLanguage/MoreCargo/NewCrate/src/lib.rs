
// rust uses documentation comment that will generate HTML documentation
// uses /// 
// run "cargo doc" to generate the documentation
// run "cargo doc --open" to open the documentation and build it
// commonly used sections
// # panic to show where it could panic
// # errors that describes what errors could occur
// # safety if unsafe explain why

// adding example code blocks also can be used as tests
// running cargo test will test the test in the example documentation code 

// //! adds documentation to the item that contains the comments rahter than to the items following the comment
// typically use these doc comments inside the crate root file or inside a module to document the crate or the modle as a whole

// structure of public api is very important
// if structure not convinenit, can re export items to make a public structure

//! # NewCrate
//!
//! `NewCrate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = NewCrate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// re-export items for public api
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}