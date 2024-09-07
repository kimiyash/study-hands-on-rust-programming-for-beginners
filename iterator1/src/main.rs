fn main() {
    let it = Iter {
        curent: 0,
        max: 10,
    };
    for num in it {
        println!("{}", num);
    }
}

struct Iter {
    curent: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.curent += 1;
        if self.curent -1 < self.max {
            Some(self.curent - 1)
        } else {
            None
        }
    }
}
