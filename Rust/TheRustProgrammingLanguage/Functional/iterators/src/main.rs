fn main() {
    // iterator allows you to perform some task on a sequence of items
    // dont have to implement the logic yourself when you use an iterator
    // iterators are lazy, they dont have an effect until you call methods that consume them

    // doesnt do anything
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();

    // separate the creation and the use of the iterator
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }

    // iterator trait and the next method
    // all iterators implement trait called "Iterator"
    /* def of trait looks like this
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    */
    // only need to implement next for the iterator trait
    // returns one item of the iterator wrapped in a Some and None when it is over

    // can call the next method directy
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); // mutable since calling next changes the state, didnt need for for loop since it took ownership and made mutable behind the scenes
    v1_iter.next(); // == 1
    //...

    // values we get from next are immutable references
    // if we want to take ownership, call "into_iter"
    // if we want mutable references, call "iter_mut"

    // iteartor trait has a number of methods that have default implementations
    // methods that call next are consuming adaptors since calling them uses up the iterator
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();  
    let total: i32 = v1_iter.sum();
    // cannot use v1_iter anymore since sum consumed it 

    let v1: Vec<i32> = vec![1, 2, 3];
    let v1: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect turns iterators in collection

    // many iterators take closures as arguments
}
