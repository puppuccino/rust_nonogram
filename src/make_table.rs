use crate::{NonoError, Puzzle, N};
use std::io::{self, Write};
use std::process::exit;

impl Puzzle {
    pub fn new() -> Self {
        let rows = Vec::new();
        let cols = Vec::new();
        Puzzle { rows, cols }
    }
}

impl Puzzle {
    pub fn make_table(&mut self) {
        // let mut rows: Vec<Vec<u8>> = vec![];
        // let mut cols: Vec<Vec<u8>> = vec![];

        println!("Please input rows");
        for i in 1..=N {
            if let Ok(numbers) = read_values(i as usize) {
                self.rows.push(numbers);
            }
        }
        println!("Please input columns");
        for i in 1..=N {
            if let Ok(numbers) = read_values(i as usize) {
                self.cols.push(numbers);
            }
        }

        // Table { rows, cols }
    }
}

fn read_values(i: usize) -> Result<Vec<u8>, NonoError> {
    let mut input: String = String::new();

    loop {
        'read_line: loop {
            print!("No. {i}: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .map_err(|_e| NonoError::InputError)?;
            if !input.trim().is_empty() {
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
