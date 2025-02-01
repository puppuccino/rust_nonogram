use std::{str::FromStr, vec};

use crate::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseCellError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellVec {
    cells: Vec<Cell>,
}

impl FromStr for CellVec {
    type Err = ParseCellError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::new();
        for c in s.chars() {
            let cell = match c {
                'C' | 'x' | 'X' => Cell::Crossed,
                'P' | 'o' | 'O' | '0' => Cell::Painted,
                '?' => Cell::Undetermined,
                _ => return Err(ParseCellError),
            };
            cells.push(cell);
        }
        Ok(CellVec { cells })
    }
}

#[cfg(test)]
mod test_str_to_cellvec {
    use super::*;

    #[test]
    fn test_str_to_cellvec() {
        let cell_vec: CellVec = "CPOX?".parse().unwrap();
        assert_eq!(
            cell_vec.cells,
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
    fn test_str_to_cellvec_error() {
        let cell_vec: Result<CellVec, ParseCellError> = "CPXZ".parse();
        assert!(cell_vec.is_err());
    }
}

fn backtrack_painted_positions(
    constraint: &mut Vec<usize>,  // それぞれのPaintedの塊の長さ
    existing: &[Cell],            // 既存盤面のスライス
    positions: &mut Vec<usize>,   // それぞれのPaintedの塊の頭の位置を記録する
    record: &mut Vec<Vec<usize>>, //
    depth: u32,                   // 再帰の深さ。0のときは特別な処理を行う。
) {
    // 再帰を使って、与えられた範囲existingのどの区間内にPaintedの塊を配置することができるかを調べていく。
    // existingの左から右へと順番にPaintedの塊を配置していく。
    // ある範囲に配置できるなら、残りの範囲に対して再帰的に同じことを行う。

    let constraint_copy = constraint.clone();

    println!(
        "<Entry> constraint: {:?}, existing: {:?}, positions: {:?}, record: {:?}, depth: {}",
        constraint, existing, positions, record, depth
    );

    if let Some(painted_len) = constraint.pop() {
        println!("constraint: {:?} -> {:?}", constraint_copy, constraint);
        for begin in 0..existing.len() {
            println!("Loop {:?}/{:?}", begin + 1, existing.len());
            let end = begin + painted_len; // semi-open interval

            // 既存の盤面の範囲を超えてPaintedの塊を配置することはできない。
            if end > existing.len() {
                println!("範囲超え");
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
                println!("右にPaintedがある");
                continue;
            }

            // exising[begin..end] にCrossedがあるなら、[begin..end]にPaintedを置くことはできない。
            if existing[begin..end].iter().any(|c| *c == Cell::Crossed) {
                println!("Crossedと重なる");
                continue;
            }

            // Paintedの塊を置いたら、その左はCrossedで確定するので、Paintedであってはならない。
            if begin != 0 && existing[begin - 1] == Cell::Painted {
                println!("左にPaintedがある");
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

        print!("全パターン終わりました。constraint: {:?}", constraint);
        constraint.push(painted_len);
        println!(" -> {:?}", constraint);
        println!(
            "<Exit> constraint: {:?}, existing: {:?}, positions: {:?}, record: {:?}, depth: {}",
            constraint, existing, positions, record, depth
        );
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

#[cfg(test)]
mod test_solve {
    use super::*;

    #[test]
    fn test_solve_0() {
        let mut constraint = vec![2];
        let existing: CellVec = "???".parse().unwrap();
        let existing: Vec<Cell> = existing.cells;
        let mut positions = Vec::new();
        let mut record = Vec::new();
        backtrack_painted_positions(&mut constraint, &existing, &mut positions, &mut record, 0);
        /*
        0 1 2
        _ O O
        O O _
         */
        assert_eq!(record, vec![vec![0], vec![1]]);
    }

    #[test]
    fn test_solve_1() {
        let mut constraint = vec![1, 1];
        let existing: CellVec = "???".parse().unwrap();
        let existing: Vec<Cell> = existing.cells;
        let mut positions = Vec::new();
        let mut record = Vec::new();
        backtrack_painted_positions(&mut constraint, &existing, &mut positions, &mut record, 0);
        /*
        0 1 2
        O _ O
         */
        assert_eq!(record, vec![vec![0, 2]]);
    }

    #[test]
    fn test_solve_2() {
        let mut constraint = vec![1, 2];
        let existing: CellVec = "????".parse().unwrap();
        let existing: Vec<Cell> = existing.cells;
        let mut positions = Vec::new();
        let mut record = Vec::new();
        backtrack_painted_positions(&mut constraint, &existing, &mut positions, &mut record, 0);
        /*
        0 1 2 3
        O _ O O
         */
        assert_eq!(record, vec![vec![0, 2]]);
    }

    #[test]
    fn test_solve_3() {
        let mut constraint = vec![2, 2];
        let existing: CellVec = "?????".parse().unwrap();
        let existing: Vec<Cell> = existing.cells;
        let mut positions = Vec::new();
        let mut record = Vec::new();
        backtrack_painted_positions(&mut constraint, &existing, &mut positions, &mut record, 0);
        /*
        0 1 2 3 4
        O O _ O O
         */
        assert_eq!(record, vec![vec![0, 3]]);
    }

    #[test]
    fn test_solve_4() {
        let mut constraint = vec![2, 3];
        let existing: CellVec = "???????".parse().unwrap();
        let existing: Vec<Cell> = existing.cells;
        let mut positions = Vec::new();
        let mut record = Vec::new();
        backtrack_painted_positions(&mut constraint, &existing, &mut positions, &mut record, 0);
        /*
        0 1 2 3 4 5 6
        O O _ O O O _
        O O _ _ O O O
        _ O O _ O O O
         */
        assert_eq!(record, vec![vec![0, 3], vec![0, 4], vec![1, 4]]);
    }

    #[test]
    fn test_solve_5() {
        let mut constraint = vec![2, 3];
        let existing: CellVec = "???x???".parse().unwrap();
        let existing: Vec<Cell> = existing.cells;
        let mut positions = Vec::new();
        let mut record = Vec::new();
        backtrack_painted_positions(&mut constraint, &existing, &mut positions, &mut record, 0);
        /*
        0 1 2 3 4 5 6
        O O _ _ O O O
        _ O O _ O O O
         */
        assert_eq!(record, vec![vec![0, 4], vec![1, 4]]);
    }

    #[test]
    fn test_solve_6() {
        let mut constraint = vec![3, 4];
        let existing: CellVec = "??????????".parse().unwrap();
        let existing: Vec<Cell> = existing.cells;
        let mut positions = Vec::new();
        let mut record = Vec::new();
        backtrack_painted_positions(&mut constraint, &existing, &mut positions, &mut record, 0);
        let mut expected = vec![
            vec![0, 4],
            vec![0, 5],
            vec![0, 6],
            vec![1, 5],
            vec![1, 6],
            vec![2, 6],
        ];
        record.sort();
        expected.sort();
        assert_eq!(record, expected);
    }

    #[test]
    fn test_solve_7() {
        let mut constraint = vec![2, 2];
        let existing: CellVec = "?????O????".parse().unwrap();
        let existing: Vec<Cell> = existing.cells;
        let mut positions = Vec::new();
        let mut record = Vec::new();
        backtrack_painted_positions(&mut constraint, &existing, &mut positions, &mut record, 0);
        let mut expected = vec![
            vec![0, 4],
            vec![0, 5],
            vec![1, 4],
            vec![1, 5],
            vec![2, 5],
            vec![4, 7],
            vec![4, 8],
            vec![5, 8],
        ];
        record.sort();
        expected.sort();
        assert_eq!(record, expected);
    }
}

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
        // どれか一つの塗り方でPaintedになるセルは、必ずしもCrossedとは言えない。
        // どれか一つの塗り方でCrossedになるセルは、必ずしもPaintedとは言えない。
        for (i, c) in pattern.iter().enumerate() {
            if *c == Cell::Painted {
                is_fixed_painted[i] = false;
            } else if *c == Cell::Crossed {
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
    updatable_cells
}

#[cfg(test)]
mod test_list_updatable_cells {
    use super::*;

    #[test]
    fn test_list_updatable_cells_0() {
        let constraint = vec![3];
        let existing: CellVec = "?????".parse().unwrap();
        let existing: Vec<Cell> = existing.cells;
        let updatable_cells = list_updatable_cells(&constraint, &existing);
        let expected = vec![(2, Cell::Crossed)];
        assert_eq!(updatable_cells, expected);
    }
}
