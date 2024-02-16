// OOP programs made up of objects
// an object packages both data and the procedures that operate on that data
// the procedures are typically called methods or operations

// encapsulation hides implementation details
// this is in rust through the pub keyword

// struct is public but the members are private
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// impl that provides methods
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// inheritance as a type system and as code sharing
// can get a similar functionality through traits
// can also enable polymorphism