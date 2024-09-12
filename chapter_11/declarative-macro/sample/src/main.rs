macro_rules! five_times {
    ( $x:expr ) => {
        5 * $x
    };
}

macro_rules! vec {
    ( $x:ty ) => {
        {
            let temp_vec: Vec<$x> = Vec::new();
            temp_vec
        }
    };
    ( $( $x:expr ),* ) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}

fn main() {
    println!("{}", five_times!(2 + 3));
    assert_eq!(25, five_times!(2 + 3));

    let x = vec![0];
    let y = vec![5, 6, 7];
    let z = vec![i32];
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
