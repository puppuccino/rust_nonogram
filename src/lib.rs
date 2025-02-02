mod alt;
pub mod make_table;

pub const N: u8 = 15;

pub struct Puzzle {
    pub rows: Vec<Vec<u8>>,
    pub cols: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub enum NonoError {
    InputError,
    ParseError,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Cell {
    Painted,      // 塗りつぶすマス, Oで表現する
    Crossed,      // 塗りつぶさないマス, Xで表現する
    Undetermined, // 未確定のマス, ?で表現する
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Painted => write!(f, "O"),
            Cell::Crossed => write!(f, "X"),
            Cell::Undetermined => write!(f, "?"),
        }
    }
}

pub fn cells_to_string(cells: Vec<Cell>) -> String {
    cells.into_iter().map(|c| c.to_string()).collect()
}

impl Cell {
    pub fn from_char(s: &char) -> Option<Cell> {
        match s {
            'C' | 'x' | 'X' => Some(Cell::Crossed),
            'P' | 'o' | 'O' | '0' => Some(Cell::Painted),
            '?' => Some(Cell::Undetermined),
            // 未定義の文字はすべて読み飛ばす
            _ => None,
        }
    }
}

pub fn string_to_cells(s: &str) -> Vec<Cell> {
    s.chars().filter_map(|c| Cell::from_char(&c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_cells_success() {
        assert_eq!(
            string_to_cells("OX?"),
            vec![Cell::Painted, Cell::Crossed, Cell::Undetermined]
        );
    }

    #[test]
    fn test_cells_to_string() {
        assert_eq!(
            cells_to_string(vec![Cell::Painted, Cell::Crossed, Cell::Undetermined]),
            "OX?"
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellVec {
    cells: Vec<Cell>,
}

impl From<&str> for CellVec {
    /// &str の各文字を `Cell::from_char` で変換し、変換できたもののみを集めます。
    fn from(s: &str) -> Self {
        let cells = s.chars().filter_map(|c| Cell::from_char(&c)).collect();
        CellVec { cells }
    }
}

#[cfg(test)]
mod test_str_to_cellvec {
    use super::*;

    #[test]
    fn test_str_to_cellvec() {
        let cell_vec: CellVec = "CPOX?".into();
        let cv = cell_vec.cells;
        assert_eq!(
            cv,
            vec![
                Cell::Crossed,
                Cell::Painted,
                Cell::Painted,
                Cell::Crossed,
                Cell::Undetermined
            ]
        );
    }

    #[test]
    fn test_str_to_cellvec_separator() {
        let cell_vec: CellVec = "C_P_O_X_?".into();
        let cv = cell_vec.cells;
        assert_eq!(
            cv,
            vec![
                Cell::Crossed,
                Cell::Painted,
                Cell::Painted,
                Cell::Crossed,
                Cell::Undetermined
            ]
        );
    }
}

fn backtrack_painted_positions(
    constraint: &mut Vec<usize>,  // それぞれのPaintedの塊の長さ
    existing: &[Cell],            // 既存盤面のスライス
    positions: &mut Vec<usize>,   // それぞれのPaintedの塊の頭の位置を記録する
    record: &mut Vec<Vec<usize>>, //
    depth: u32,                   // 再帰の深さ。デバッグ時に見やすいように追加した。
) {
    // 再帰を使って、与えられた範囲existingのどの区間内にPaintedの塊を配置することができるかを調べていく。
    // existingの左から右へと順番にPaintedの塊を配置していく。
    // ある範囲に配置できるなら、残りの範囲に対して再帰的に同じことを行う。

    if let Some(painted_len) = constraint.pop() {
        for begin in 0..existing.len() {
            let end = begin + painted_len; // semi-open interval

            // 既存の盤面の範囲を超えてPaintedの塊を配置することはできない。
            if end > existing.len() {
                // constraint.push(painted_len);
                continue;
            }

            /*
            置くPaintedの塊よりも右には既にPaintedがあってはならない。

            例: パズルサイズは10とする。
            0 1 2 3 4 5 6 7 8 9
            _ _ _ _ _ _ _ _ _ _
            いま仮にbegin=3, len=3とする(end=6)と、 [3..6]にPainted(Oで表す)を置くには
            existing[6..]がすべてUndetermined(?)またはCrossed(X)である必要がある。

            0 1 2 3 4 5 6 7 8 9
            _ _ _ O O O ? ? ? ? <- これは可能
            _ _ _ O O O ? X X ? <- これも可能
            _ _ _ O O O ? O O ? <- これは不可能

            */
            if existing[end..].iter().any(|c| *c == Cell::Painted) {
                continue;
            }

            // exising[begin..end] にCrossedがあるなら、[begin..end]にPaintedを置くことはできない。
            if existing[begin..end].iter().any(|c| *c == Cell::Crossed) {
                continue;
            }

            // Paintedの塊を置いたら、その左はCrossedで確定するので、Paintedであってはならない。
            if begin != 0 && existing[begin - 1] == Cell::Painted {
                continue;
            }

            // いずれも問題なければ、[begin..end]にPaintedの塊を配置できる。
            positions.push(begin);

            if begin == 0 {
                backtrack_painted_positions(
                    constraint,
                    &existing[..begin],
                    positions,
                    record,
                    depth + 1,
                );
            } else {
                backtrack_painted_positions(
                    constraint,
                    &existing[..begin - 1],
                    positions,
                    record,
                    depth + 1,
                );
            }

            positions.pop();
        }

        constraint.push(painted_len);
    } else {
        // constraintが空ということは、すべてのPaintedの塊が配置されたということ。
        // そのため、残った範囲内にPaintedがあってはならない。
        if existing.iter().any(|c| *c == Cell::Painted) {
            return;
        }
        // もし見つからなければ、全てのPaintedの塊が問題なく配置されたということなので、recordに記録する。
        // ただし、先に置いた塊がpositionsの前に来ているので、positionsを逆順にしたうえでrecordにpushする。
        record.push(positions.iter().copied().rev().collect());
    }
}

/// 与えられた制約と既存の盤面から、Paintedを置くことができる位置を探す。
/// # Arguments
/// - constraint: それぞれのPaintedの塊の長さ
/// - existing: 既存の盤面
/// # Returns:
/// - それぞれのPaintedの塊の頭の位置を記録したVec
pub fn find_paintable_positions(constraint: &Vec<usize>, existing: &[Cell]) -> Vec<Vec<usize>> {
    let mut positions = Vec::new();
    let mut record = Vec::new();
    let constraint = constraint.clone();
    backtrack_painted_positions(
        &mut constraint.clone(),
        existing,
        &mut positions,
        &mut record,
        0,
    );
    record
}

#[cfg(test)]
mod test_find_paintable_positions {
    use super::*;

    fn run_test(constraint: Vec<usize>, existing: &str, expected: Vec<Vec<usize>>) {
        let existing: CellVec = existing.into();
        let mut actual = find_paintable_positions(&constraint, &existing.cells);
        actual.sort();
        let mut expected = expected;
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_0() {
        run_test(vec![2], "???", vec![vec![0], vec![1]]);
    }

    #[test]
    fn test_solve_1() {
        run_test(vec![1, 1], "???", vec![vec![0, 2]]);
    }

    #[test]
    fn test_solve_2() {
        run_test(vec![1, 2], "????", vec![vec![0, 2]]);
    }

    #[test]
    fn test_solve_3() {
        run_test(vec![2, 2], "?????", vec![vec![0, 3]]);
    }

    #[test]
    fn test_solve_4() {
        run_test(
            vec![2, 3],
            "???????",
            vec![vec![0, 3], vec![0, 4], vec![1, 4]],
        );
    }

    #[test]
    fn test_solve_5() {
        run_test(vec![2, 3], "???x???", vec![vec![0, 4], vec![1, 4]]);
    }

    #[test]
    fn test_solve_6() {
        run_test(
            vec![3, 4],
            "?????_?????",
            vec![
                vec![0, 4],
                vec![0, 5],
                vec![0, 6],
                vec![1, 5],
                vec![1, 6],
                vec![2, 6],
            ],
        );
    }

    #[test]
    fn test_solve_7() {
        run_test(
            vec![2, 2],
            "?????O????",
            vec![
                vec![0, 4],
                vec![0, 5],
                vec![1, 4],
                vec![1, 5],
                vec![2, 5],
                vec![4, 7],
                vec![4, 8],
                vec![5, 8],
            ],
        );
    }

    #[test]
    fn test_solve_8() {
        run_test(
            vec![3, 3],
            "?????_x????",
            vec![
                vec![0, 6],
                vec![0, 7],
                vec![1, 6],
                vec![1, 7],
                vec![2, 6],
                vec![2, 7],
            ],
        );
    }
}

fn list_updatable_cells(constraint: &Vec<usize>, existing: &[Cell]) -> Vec<(usize, Cell)> {
    let head_positions = find_paintable_positions(constraint, existing);

    // true: そのセルはCrossedで確定する, false: そのセルはUndeterminedのままである。
    let mut is_fixed_crossed: Vec<bool> = vec![true; existing.len()];
    // true: そのセルはPaintedで確定する, false: そのセルはUndeterminedのままである。
    let mut is_fixed_painted: Vec<bool> = vec![true; existing.len()];

    for positions in &head_positions {
        let mut pattern = vec![Cell::Crossed; existing.len()];
        for (i, p) in positions.iter().enumerate() {
            for j in 0..constraint[i] {
                pattern[p + j] = Cell::Painted;
            }
        }
        // Crossedで確定するには、すべての塗り方でCrossedである必要がある。
        // Paintedで確定するには、すべての塗り方でPaintedである必要がある。
        for (i, c) in pattern.iter().enumerate() {
            if *c == Cell::Crossed {
                is_fixed_painted[i] = false;
            } else if *c == Cell::Painted {
                is_fixed_crossed[i] = false;
            }
        }
    }

    let mut updatable_cells = Vec::new();
    for i in 0..existing.len() {
        if is_fixed_crossed[i] {
            updatable_cells.push((i, Cell::Crossed));
        } else if is_fixed_painted[i] {
            updatable_cells.push((i, Cell::Painted));
        }
    }

    // すでに確定していたセルは除外する
    updatable_cells.retain(|(i, _)| existing[*i] == Cell::Undetermined);
    updatable_cells
}

#[cfg(test)]
mod test_list_updatable_cells {
    use super::*;

    fn run_test(constraint: Vec<usize>, existing: &str, expected: Vec<(usize, char)>) {
        let existing: CellVec = existing.into();
        let existing: Vec<Cell> = existing.cells;
        let updatable_cells = list_updatable_cells(&constraint, &existing);
        let expected: Vec<(usize, Cell)> = expected
            .into_iter()
            .map(|(i, c)| (i, Cell::from_char(&c).unwrap()))
            .collect();
        assert_eq!(updatable_cells, expected);
    }

    #[test]
    fn test_list_updatable_cells_0() {
        run_test(vec![3], "?????", vec![(2, 'O')]);
    }

    #[test]
    fn test_list_updatable_cells_1() {
        run_test(vec![8], "?????_?????_?????", vec![(7, 'O')]);
    }

    #[test]
    fn test_list_updatable_cells_2() {
        run_test(
            vec![2, 2],
            "?????",
            vec![(0, 'O'), (1, 'O'), (2, 'X'), (3, 'O'), (4, 'O')],
        );
    }

    #[test]
    fn test_list_updatable_cells_3() {
        run_test(vec![2, 2], "?????_O????", vec![]);
    }

    #[test]
    fn test_list_updatable_cells_4() {
        run_test(
            vec![2, 2, 7],
            "?????_?????_?????",
            vec![(8, 'O'), (9, 'O'), (10, 'O'), (11, 'O'), (12, 'O')],
        );
    }

    #[test]
    fn test_list_updatable_cells_5() {
        run_test(
            vec![5, 2, 5],
            "?????_?????_?????",
            vec![
                (1, 'O'),
                (2, 'O'),
                (3, 'O'),
                (4, 'O'),
                (7, 'O'),
                (10, 'O'),
                (11, 'O'),
                (12, 'O'),
                (13, 'O'),
            ],
        );
    }

    #[test]
    fn test_list_updatable_cells_6() {
        run_test(
            vec![1, 8, 2],
            "?????_?????_?????",
            vec![(4, 'O'), (5, 'O'), (6, 'O'), (7, 'O'), (8, 'O'), (9, 'O')],
        );
    }

    #[test]
    fn test_list_updatable_cells_7() {
        run_test(
            vec![4, 3, 1, 1],
            "?????_?????_xxox?",
            vec![(2, 'O'), (3, 'O'), (7, 'O')],
        );
    }

    #[test]
    fn test_list_updatable_cells_8() {
        run_test(
            vec![5, 2, 1, 1],
            "?????_??o??_????o",
            vec![(3, 'O'), (4, 'O'), (13, 'X')],
        );
    }

    #[test]
    fn test_list_updatable_cells_9() {
        run_test(
            vec![3, 3],
            "?????_x????",
            vec![(2, 'O'), (7, 'O'), (8, 'O')],
        );
    }

    #[test]
    fn test_list_updatable_cells_10() {
        run_test(
            vec![4, 1, 1],
            "?????_x?x??",
            vec![(1, 'O'), (2, 'O'), (3, 'O'), (6, 'O')],
        );
    }
}
