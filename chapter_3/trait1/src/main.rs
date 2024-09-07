trait Tweet {
    fn tweet(&self);
    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }
    fn shout(&self) {
        println!("Uooooooooooo!");
    }
}

struct Duck;
struct Dove;

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack");
    }
    
}

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo")
    }
}

struct Dropable;

impl Drop for Dropable {
    fn drop(&mut self) {
        println!("Resource will be released!");
    }
}

fn main() {
    let dove = Dove;
    let duck = Duck;

    dove.tweet();
    println!("----");
    dove.tweet_twice();
    println!("----");
    dove.shout();

    println!("");

    duck.tweet();
    println!("----");
    duck.tweet_twice();
    println!("----");
    dove.shout();

    println!("");

    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }

    println!("");
    {
        let _dropable = Dropable;        
    }
    println!("dropped")
}
