#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    fn new(name: String, age: u32, weight: f32) -> User {
        User {
            name: name,
            age: age,
            weight: weight,
        }
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}

fn main() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    bob.weight();
    bob.set_age(33);
    println!("{:?}", bob);
}
