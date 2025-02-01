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

impl std::str::FromStr for Cell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O" => Ok(Cell::Painted),
            "X" => Ok(Cell::Crossed),
            "?" => Ok(Cell::Undetermined),
            _ => Err(()),
        }
    }
}

use std::str::FromStr;

pub fn string_to_cells(s: &str) -> Result<Vec<Cell>, ()> {
    // 1文字でも変換できない場合はエラーを返す。全文字変換できたらOkで返す
    s.chars().map(|c| Cell::from_str(&c.to_string())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_cells_success() {
        assert_eq!(
            string_to_cells("OX?").unwrap(),
            vec![Cell::Painted, Cell::Crossed, Cell::Undetermined]
        );
    }

    #[test]
    fn test_string_to_cells_fail() {
        assert!(string_to_cells("OX?A").is_err());
    }

    #[test]
    fn test_cells_to_string() {
        assert_eq!(
            cells_to_string(vec![Cell::Painted, Cell::Crossed, Cell::Undetermined]),
            "OX?"
        );
    }
}

use std::collections::HashSet;

/// 制約をもとに、ありうるマス目を列挙する
/// # Arguments
/// * `constraint` - 制約
/// * `n` - パズルサイズ
/// # Returns
/// * ありうるマス目のセット
fn enumerate_cells(constraint: &[u8], n: u8) -> Result<HashSet<Vec<Cell>>, NonoError> {
    /*
    例えば、制約が[3,2]でパズルサイズが8の場合を考える。
    // Oの塊(この場合OOOとOO)の左右または中間にXを挿入することで、ありうるマス目を列挙する。
    |OOO|XOO| の|の位置に0個以上のXを挿入することが可能で、合計の個数n_xはパズルサイズ-(制約の合計+制約の個数-1)
    ここで、制約の個数-1というのは、Oの塊の間には必ず1つ以上のXが入るため。
    この場合、8-(5+2-1)=2個のXを3箇所の|の位置に挿入することが可能で、全部で3H2=4C2=6通りのマス目がありうる：
    XXOOOXOO, XOOOXXOO, XOOOXOOX, OOOXXXOO, OOOXXOOX, OOOXOOXX
    */

    // まず、制約の合計を計算する
    let sum: u8 = constraint.iter().sum();
    // 制約の個数-1 を足す
    let sum = sum + (constraint.len() as u8) - 1;
    // これはパズルサイズを超えてはいけない
    if sum > n {
        return Err(NonoError::InputError);
    }

    let n_x = n - sum;
    let n_pos = constraint.len() + 1;
    let mut pat: Vec<Vec<u8>> = helper(n_pos, n_x);
    // patの中の各Vec<u8>の先頭と末尾以外に1を足すと、Oの塊の間に入るXの個数のリストになる。
    for v in pat.iter_mut() {
        for i in 1..v.len() - 1 {
            v[i] += 1;
        }
    }

    let mut ret = HashSet::new();
    for p in pat {
        let mut cells = vec![];
        for i in 0..constraint.len() {
            cells.extend(vec![Cell::Crossed; p[i] as usize]);
            cells.extend(vec![Cell::Painted; constraint[i] as usize]);
        }
        cells.extend(vec![Cell::Crossed; p[n_pos - 1] as usize]);

        ret.insert(cells);
    }

    Ok(ret)
}

#[cfg(test)]
mod enumerate_cells_tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_enumerate_cells_size10_4_5() {
        assert_eq!(
            enumerate_cells(&vec![4, 5], 10).unwrap(),
            HashSet::from_iter(vec![string_to_cells("OOOOXOOOOO").unwrap()])
        );
    }
    #[test]
    fn test_enumerate_cells_size10_1_8() {
        assert_eq!(
            enumerate_cells(&vec![1, 8], 10).unwrap(),
            HashSet::from_iter(vec![string_to_cells("OXOOOOOOOO").unwrap()])
        );
    }
    #[test]
    fn test_enumerate_cells_size8_3_2() {
        assert_eq!(
            enumerate_cells(&vec![3, 2], 8).unwrap(),
            HashSet::from_iter(vec![
                string_to_cells("XXOOOXOO").unwrap(),
                string_to_cells("XOOOXXOO").unwrap(),
                string_to_cells("XOOOXOOX").unwrap(),
                string_to_cells("OOOXXXOO").unwrap(),
                string_to_cells("OOOXXOOX").unwrap(),
                string_to_cells("OOOXOOXX").unwrap(),
            ])
        );
    }

    #[test]
    fn test_enumerate_cells_size10_2_2_3() {
        assert_eq!(
            enumerate_cells(&vec![2, 2, 3], 10).unwrap(),
            HashSet::from_iter(vec![
                string_to_cells("OOXOOXOOOX").unwrap(),
                string_to_cells("OOXOOXXOOO").unwrap(),
                string_to_cells("OOXXOOXOOO").unwrap(),
                string_to_cells("XOOXOOXOOO").unwrap(),
            ])
        );
    }

    #[test]
    fn test_enumerate_cells_size10_1_2_3() {
        assert_eq!(
            enumerate_cells(&vec![1, 2, 3], 10).unwrap(),
            HashSet::from_iter(vec![
                string_to_cells("OXOOXOOOXX").unwrap(),
                string_to_cells("OXOOXXOOOX").unwrap(),
                string_to_cells("OXXOOXOOOX").unwrap(),
                string_to_cells("XOXOOXOOOX").unwrap(),
                string_to_cells("OXOOXXXOOO").unwrap(),
                string_to_cells("OXXOOXXOOO").unwrap(),
                string_to_cells("XOXOOXXOOO").unwrap(),
                string_to_cells("OXXXOOXOOO").unwrap(),
                string_to_cells("XOXXOOXOOO").unwrap(),
                string_to_cells("XXOXOOXOOO").unwrap(),
            ])
        );
    }

    #[test]
    fn test_enumerate_cells_size15_8_1() {
        assert_eq!(
            enumerate_cells(&vec![8, 1], 15).unwrap(),
            HashSet::from_iter(vec![
                string_to_cells("OOOOOOOOXOXXXXX").unwrap(),
                string_to_cells("OOOOOOOOXXOXXXX").unwrap(),
                string_to_cells("XOOOOOOOOXOXXXX").unwrap(),
                string_to_cells("OOOOOOOOXXXOXXX").unwrap(),
                string_to_cells("XOOOOOOOOXXOXXX").unwrap(),
                string_to_cells("XXOOOOOOOOXOXXX").unwrap(),
                string_to_cells("OOOOOOOOXXXXOXX").unwrap(),
                string_to_cells("XOOOOOOOOXXXOXX").unwrap(),
                string_to_cells("XXOOOOOOOOXXOXX").unwrap(),
                string_to_cells("XXXOOOOOOOOXOXX").unwrap(),
                string_to_cells("OOOOOOOOXXXXXOX").unwrap(),
                string_to_cells("XOOOOOOOOXXXXOX").unwrap(),
                string_to_cells("XXOOOOOOOOXXXOX").unwrap(),
                string_to_cells("XXXOOOOOOOOXXOX").unwrap(),
                string_to_cells("XXXXOOOOOOOOXOX").unwrap(),
                string_to_cells("OOOOOOOOXXXXXXO").unwrap(),
                string_to_cells("XOOOOOOOOXXXXXO").unwrap(),
                string_to_cells("XXOOOOOOOOXXXXO").unwrap(),
                string_to_cells("XXXOOOOOOOOXXXO").unwrap(),
                string_to_cells("XXXXOOOOOOOOXXO").unwrap(),
                string_to_cells("XXXXXOOOOOOOOXO").unwrap(),
            ])
        );
    }
}

/// 長さlで合計がsのリストを列挙する
fn helper(l: usize, s: u8) -> Vec<Vec<u8>> {
    if l == 1 {
        return vec![vec![s]];
    }

    let mut ret = vec![];
    for i in 0..=s {
        for mut v in helper(l - 1, s - i) {
            v.push(i);
            ret.push(v);
        }
    }
    ret.sort();
    ret
}

#[cfg(test)]
mod helper_tests {
    use super::*;

    #[test]
    fn test_helper_1() {
        assert_eq!(
            helper(2, 3),
            vec![vec![0, 3], vec![1, 2], vec![2, 1], vec![3, 0]]
        );
    }

    #[test]
    fn test_helper_2() {
        assert_eq!(
            helper(3, 3),
            vec![
                vec![0, 0, 3],
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![0, 3, 0],
                vec![1, 0, 2],
                vec![1, 1, 1],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0],
                vec![3, 0, 0]
            ]
        );
    }
}

/// `candidate` と `existing` のセルが矛盾しないかをチェックする関数
/// # Arguments
/// * `candidate` - 候補となるセルのベクトル
/// * `existing` - 既存のセルのベクトル
/// # Returns
/// 矛盾しない場合は `true`、矛盾する場合は `false` を返す
fn is_consistent(candidate: &[Cell], existing: &[Cell]) -> bool {
    for (c, e) in candidate.iter().zip(existing.iter()) {
        match (c, e) {
            (Cell::Painted, Cell::Crossed) => return false,
            (Cell::Crossed, Cell::Painted) => return false,
            _ => (),
        }
    }
    true
}

#[derive(Debug, PartialEq)]
pub struct Delta {
    idx: usize,
    kind: Cell,
}

pub fn list_updatable_cells(pattern: &HashSet<Vec<Cell>>, existing: &[Cell]) -> Vec<Delta> {
    if pattern.is_empty() {
        return Vec::new();
    }

    let mut pattern = pattern.clone();
    pattern.retain(|p| is_consistent(p, existing));

    // 残ったパターン全てに共通するセルを列挙する
    let first = pattern.iter().next().unwrap();
    let len = first.len();
    let mut ret = vec![];

    for i in 0..len {
        let c = first[i].clone();
        if pattern.iter().all(|p| p[i] == c) {
            ret.push(Delta { idx: i, kind: c });
        }
    }

    ret
}

mod list_updatable_cells_test {
    use super::*;

    #[test]
    fn test_list_updatable_cells_1() {
        let pattern = enumerate_cells(&vec![3, 2], 8).unwrap();
        let existing = string_to_cells("????????").unwrap();
        let actual = list_updatable_cells(&pattern, &existing);
        let expected = vec![Delta {
            idx: 2,
            kind: Cell::Painted,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_list_updatable_cells_2() {
        let pattern = enumerate_cells(&vec![3, 4], 10).unwrap();
        let existing = string_to_cells("??????????").unwrap();
        let actual = list_updatable_cells(&pattern, &existing);
        let expected = vec![
            Delta {
                idx: 2,
                kind: Cell::Painted,
            },
            Delta {
                idx: 6,
                kind: Cell::Painted,
            },
            Delta {
                idx: 7,
                kind: Cell::Painted,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_list_updatable_cells_3() {
        let pattern = enumerate_cells(&vec![3, 4], 10).unwrap();
        let existing = string_to_cells("??????????").unwrap();
        let actual = list_updatable_cells(&pattern, &existing);
        let expected = vec![
            Delta {
                idx: 2,
                kind: Cell::Painted,
            },
            Delta {
                idx: 6,
                kind: Cell::Painted,
            },
            Delta {
                idx: 7,
                kind: Cell::Painted,
            },
        ];
        assert_eq!(actual, expected);
    }
}
