struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn main() {
    // drop trait lets you customize what happends when a value is about to do out of scope
    // almost always used when implementing smart pointers
    // requires you to implements one method named drop that takes a mutable referenc eto self

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    // use std::mem::drop to force a drop early
    drop(c);

}
