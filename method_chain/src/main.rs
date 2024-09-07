struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person{
        Person {
            name: String::from(name),
            age,
        }
    }

    fn say_name(&self) -> &Self {
        println!("I am {}", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} years old", self.age);
        self
    }
    
}

fn main() {
    let person = Person::new("taro", 8);
    person.say_name().say_age();
}
