use std::fmt::Display;

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: String,
    retweet: String,
}

fn main() {
    // trait defines functionality a type can share with other types
    // trait bound to specify that a generic type can be any type that has a certain behavior

    // defining a trait
    // trait definitions are a way to group method signatures together to define a set of behaviors
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// defining a trait
pub trait Summary {
    fn summarize_author(&self) -> String; // just have to implement this to call summarize 
    // will be the default behavior
    // can call other methods in the same trait
    fn summarize(&self) -> String {
        String::from("Read more... {}", self.summarize_author())
    }
}

// implement trait on a struct
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// can make it so a parameter has to have a sepcific trait
// syntax sugar for a trait bound
// fn notify<T: Summary>(item: &T) {}
fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize());
}

// can also specify multiple trait bounds
fn notify(item: &(impl Summary + Display)) {}
fn notify<T: Summary + Display>(item: &T) {}

// cleaner trait bounds with where
// instead of 
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// can use a where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone
    U: Clone + Debug
{}

// can also return types that implement traits
// returns some type that impelements summary
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// useing trait bounts to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// only implemented if T has display and partialord traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


