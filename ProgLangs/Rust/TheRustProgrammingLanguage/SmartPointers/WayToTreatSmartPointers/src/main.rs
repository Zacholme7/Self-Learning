use std::ops::Deref;

fn main() {
    // treating smart pointers like regular references with the deref trait
    // implementing deref allows you to customize the behavior of the dereference operator
    // can write code that operates on both references and smart pointers

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // follows the reference to its value


    // difference here is that y is pointing to a ocpied value of x rather than referring to x
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); 

    // using MyBox
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // behind the scenes *(y.deref())

    // deref coercion converts a reference to a type that impelemnts the deref trait into a reference to another type
    // Ex: convert &String to &str
    let m = MyBox::new(String::from("rust"));
    hello(&m); // deref coercsion makes this call possible
    // since we have deref on MyBox<T>, rust can turn &MyBox<String> into &String by calling deref
    // the standard library then provides implementation of Deref on STring that returns a string slice, &String -> &str

    // how deref coercion interacts with mutability
    // can use ReferMut to override * on mutable references
    // does deref coercsion when it finds types and trait implementaions in three cases
        // From &T to &U when T: Deref<Target=U>
        // From &mut T to &mut U when T: DerefMut<Target=U>
        // From &mut T to &U when T: Deref<Target=U>
    



}

// building own smart pointer similar to Box<T>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


impl<T> Deref for MyBox<T> {
    type Target = T; // defines an associated type of the deref trait to use

    fn deref(&self) -> &Self::Target {
        &self.0 // accesses first value in the tuple struct
    }
}


// deref coercion in action
fn hello(name: &str) {
    println!("Hello {name}");
}