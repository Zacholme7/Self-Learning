// sometiems we want to be able to extend the set of types that are valid in a particular situation
// to ensure behavior is implemented, define a trait 

// define a trait with a method
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // vector that takes objects that implement the draw trait
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// represent something that implements the draw trait and can be added to components
pub struct Button {
    pub width: u32,
    pub height: u32, 
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw a button
    }
}

// trait objects do not work well with type inference


