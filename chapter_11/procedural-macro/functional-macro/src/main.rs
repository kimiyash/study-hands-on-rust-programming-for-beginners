use tomlstruct::tomlstruct;

tomlstruct! {
    [Hello]
    name = "hello"
    version = 1.0
}

fn main() {
    let x = Hello {
        name: String::from("hello"),
        version: 1.0,
    };
    println!("{:?}", x);
}
