fn main() {
        // a crate is the smallest amount of code that a rust compiler considers at a time
    // crates can contain modules
    // modules may be defined in other files that get compiled with the crate
    // can have binary crate or library crate
    // binary crate are programs you can compile and execute
        // must have main function which defines what happens when main runs
    // library crates dont have main and they dont compile to executable
        // define funtionality intended to be shared
    // crate root is source file that the rust compiler starts from and makes up the root module of the crate
    // package is a bundle of one or more crates that provide functionality
        // contains cargo.toml file that describes how to build teh crates
    // package can have multiple binary crates, but only one library crate
    // convention that src/main.rs is crate root for binary and src/lib.rs for library

    // modules cheat sheet
    // start from the crate root: it is what the compiler looks at first
    // declaring modules: in crate root, you can declare new modules
        // compiler will look for modules in these places
            // inline, in file src/garden.rs, in file src/garden/mod.rs
    // declaring submodules: in file other than crate root, can declare submodules
        // compile will look for submodules in these places
            // inline, in file src/garden/vegetables.rs, in file src/garden/vegetables/mod.rs
    // paths to code in modules: once module in crate, can refer to code anywhere in the same crate
    // private vs public: code within module is private from its parent modules by default
        // declare with pub mod to make it public
    // the use keyword: creates shortcuts to items to reduce repetition of long paths

    use crate::garden::vegetables::Asparagus;

    pub mod garden;
    let plant = Asparagus {};
    println!("Im growing {:?}", plant);
}
