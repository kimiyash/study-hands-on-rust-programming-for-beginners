use funclog::funclog;

#[funclog]
fn hello() {
    println!("Hello, world!");
}

#[funclog]
fn hello2() {
    println!("Hello, world2!");
}

fn main() {
    hello();
    hello2();
}