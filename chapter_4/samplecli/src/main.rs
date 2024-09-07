use clap::{App, Arg};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

struct RpnCalcurator(bool);

impl RpnCalcurator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
    }
}

fn main() {
    let matches = App::new("My RPN program")
        .version("1.0.0")
        .author("kimiyash")
        .about("Super awsome sample RPN calcurator")
        .arg(
            Arg::new("formula_file")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches();

    let verbose = matches.is_present("verbose");

    if let Some(path) = matches.value_of("formula_file") {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, verbose);
    }

}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalcurator::new(verbose);
    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}
