#[link(name = "hello")]
extern "C" {
    fn c_hello();
    fn c_fib(n: u32) -> u32;
}

fn main() {
    println!("Hello, world from Rust!");
    unsafe {
        println!("{}", c_fib(45));
        c_hello();
    }
}
