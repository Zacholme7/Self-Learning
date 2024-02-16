use std::io::stdin;

// debug placeholders: {:?} for raw printing and {:#?} for pretty printing
#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote {note : String},
    Refuse,
    Probation,
}

// implmenet functions for a struct with impl and the struct name
impl Visitor {
    // associated fucntion since param list does not include self, vall with Visitor::new
    // Self refers to the struct type itself, self refers to the instance of the struct
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    //  member function or method since it accepts self as a parameter
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}


fn main() {
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new("Steve", VisitorAction::AcceptWithNote{
            note: String::from("Lactose-free milk is in the fridge")
        }, 15),
        Visitor::new("Fred", VisitorAction::Refuse, 30),
    ];
  

    loop {
        println!("Hello, what is your name?");
        let name = get_name();

        let known_visitor = visitor_list
            .iter() // turns the list into an iterator
            .find(|visitor| visitor.name == name); // run a closure that returns the matching value if it is true

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() { // check if nothing was typed in
                    break;
                } else {
                    // add to the list
                    println!("{} is not on the visitor list", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);

}


fn get_name() -> String {
    let mut your_name = String::new();

    stdin() // returns object grading access to standard input
        .read_line(&mut your_name) // mutable borrows string, read line receives keyboard input until enter is pressed and puts it into the string
        .expect("Failed to read line"); // unwrap the result and fail with message if it is an error

    your_name
        .trim() // get rid of whitespace
        .to_lowercase() // turns the string lowercase
}
