pub mod make_table;

pub const N: u8 = 15;

pub struct Puzzle {
    pub rows: Vec<Vec<u8>>,
    pub cols: Vec<Vec<u8>>,
}

pub enum NonoError {
    InputError,
    ParseError,
}
