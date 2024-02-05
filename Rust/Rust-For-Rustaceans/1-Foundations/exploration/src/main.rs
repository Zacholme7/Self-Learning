fn main() {

    let x = 42; // value of type i32
    let y = 43; // value of type i32
    let var1 = &x; // pointer to x 
    let mut var2 = &x; // pointer to x
    var2 = &y;
    let string = "hello world"; // actual val of string is a pointer to the first character value
    
    let mut x;
    // this access would be illegal since we have nowhere to draw the flow from
    // assert_eq!(x, 42);
    // this is where the flow starts
    x = 42;
    // this is okay since we can draw the flow from the variable above
    let y = &x;
    // this establishes a second mutable flow from x
    x = 43; 
    // this continues the flow from y, which is turn draws from x
    // that flow conflicts with the assignment to x
    //assert_eq!(*y, 42);
    
    let x1 = 42;
    let y1 = Box::new(42);
    { // start of a new scope
        let z = (x1, y1); // move x1 and y1 here
        // z  goes out of scope and it is dropped, therefore x1, and y1 are also dropped
    }
    let x2 = x1; // x1 is copy so it was not moved into z and copied, therefore we can still use it
    // let y2 = y1; box is not copy so y1 was dropped when it was moved 




}
