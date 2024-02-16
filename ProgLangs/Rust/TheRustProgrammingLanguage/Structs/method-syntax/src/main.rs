#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// everything within impl will be associated with the rectangle type
// all functions in impl are called associated functions
// associated with the type after the impl
// can have multiple impl blocks
impl Rectangle {
    fn area(&self) -> u32 { // &self is shorthand for self: &Self, alias for the type that the impl is for
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height)
        }
    }
}


fn main() {
    // methods are similar to functions, but they are defined in the context of a struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("{}", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // ownership with structs and implementation
    let rect = Rectangle { // rect: r/o, rect.width: r/o, rect.height: r/o
        width: 0,
        height: 0,
    };

    rect.area(); // can call

    // rect.set_width(10); cannot call since we dont have write permission and arg to set_width is &mut self

    let mut rect = Rectangle { // rect: r/w/o, rect.width: r/w/o, rect.height: r/w/o
        width: 0,
        height: 0,
    };

    rect.set_width(1); // this is okay, we have write permission
    let rect_ref = &rect; // rect and width/height lose all permissions, rect_ref: r/o, width/height: r
    // rect_ref.set_width(10); will not work, do not have write permissions

    // calling a method that expects self will move input struct unless it implements copy
}

