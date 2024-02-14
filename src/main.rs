// mod my_lib;
use util::N;

use crate::util::get_table;

fn main() {
    println!("Hello, unko!");
    println!("{}", N);
    let t = get_table();
    print!("{:?}", t.rows);
}

enum NonoError {
    InputError,
    ParseError,
}

mod util {
    use crate::NonoError;
    use std::io::{self, Write};
    use std::process::exit;
    use std::vec;

    pub const N: usize = 15;
    // pub const N: usize = 3;

    pub struct Table {
        pub rows: Vec<Vec<u8>>,
        pub cols: Vec<Vec<u8>>,
    }

    pub fn read_values(i: usize) -> Result<Vec<u8>, NonoError> {
        let mut input: String = String::new();

        loop {
            'read_line: loop {
                print!("No. {i}: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .map_err(|_e| NonoError::InputError)?;
                if !input.is_empty() {
                    break 'read_line;
                }
            }

            if ["q", "quit", "exit"].contains(&input.trim()) {
                exit(1);
            }

            let v = input
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u8>())
                .collect::<Result<Vec<u8>, _>>()
                .map_err(|_e| NonoError::ParseError);

            if v.is_ok() {
                break v;
            }
        }
    }

    pub fn get_table() -> Table {
        let mut rows: Vec<Vec<u8>> = vec![];
        let mut cols: Vec<Vec<u8>> = vec![];

        println!("Please input rows");
        for i in 1..=N {
            if let Ok(numbers) = read_values(i) {
                rows.push(numbers);
            }
        }
        println!("Please input columns");
        for i in 1..=N {
            if let Ok(numbers) = read_values(i) {
                cols.push(numbers);
            }
        }

        Table { rows, cols }
    }
}
