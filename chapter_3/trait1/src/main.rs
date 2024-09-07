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
}
