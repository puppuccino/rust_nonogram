use rust_nonogram::{Puzzle, N};

fn main() {
    println!("start main");
    println!("{}", N);
    let mut puzzle = Puzzle::new();
    puzzle.make_table();
    // let t = get_table();
    // print!("{:?}", t.rows);
}
