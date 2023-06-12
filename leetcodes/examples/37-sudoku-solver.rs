#![allow(unused)]
// PTA
use std::{
    fmt::{Debug, Display, Write},
    io::{stdin, BufRead, Stdin},
    str::FromStr,
};
// leetcode
//
// use std::{
//     fmt::{Debug, Display, Write as LCWrite},
//     io::stdin,
//     str::FromStr,
// };

// 什么中英文混合编程 🫠🫠🫠🫠🫠
// 做运筹学送 leetcode: https://leetcode.com/problems/sudoku-solver/

// 来整点类型体操
trait Element: PartialEq + Display + FromStr + Debug + Copy {
    fn from_integer(i: u32) -> Self;
    fn zero() -> Self;
}

struct 数独<'a, T: Element>
where
    <T as FromStr>::Err: Debug,
{
    board: &'a mut Vec<Vec<T>>,
    feasible: bool,
}

impl Element for u32 {
    #[inline]
    fn from_integer(i: u32) -> u32 {
        i
    }
    #[inline]
    fn zero() -> u32 {
        0
    }
}

impl Element for char {
    #[inline]
    fn from_integer(i: u32) -> char {
        return char::from_digit(i, 10).unwrap();
    }
    #[inline]
    fn zero() -> char {
        return '.';
    }
}

const N: usize = 9;

impl<'a, T: Element> 数独<'a, T>
where
    <T as FromStr>::Err: Debug,
{
    fn new(board: &'a mut Vec<Vec<T>>) -> Self {
        Self {
            board,
            feasible: false,
        }
    }
    fn construct_board_from_stdin() -> Vec<Vec<T>> {
        // 辣鸡 PTA 使用远古版本 rust
        stdin()
            .lock()
            .lines()
            .map(|s| {
                s.unwrap()
                    .split_whitespace()
                    .map(|s| str::parse::<T>(s).unwrap())
                    .collect()
            })
            .collect()
    }

    #[inline]
    fn 我可以放在这里吗(&self, row: usize, col: usize, val: T) -> bool {
        let (sr, sc) = (row - row % 3, col - col % 3);
        for i in sr..sr + 3 {
            for j in sc..sc + 3 {
                if self.board[i][j] == val {
                    return false;
                }
            }
        }
        let row_cond = !self.board[row].contains(&val);
        let col_cond = !(0..9).any(|i| self.board[i][col] == val);
        row_cond && col_cond
    }

    fn 求解喵(&mut self, row: usize, col: usize) -> bool {
        // RECURSION BASE CASE
        let zero = T::zero();
        if row == N - 1 && col == N {
            return true;
        }
        // 换行
        let (row, col) = if col == N { (row + 1, 0) } else { (row, col) };
        if self.board[row][col] != zero {
            return self.求解喵(row, col + 1);
        }
        (1..=9).any(|v| {
            let val = T::from_integer(v);
            if self.我可以放在这里吗(row, col, val) {
                self.board[row][col] = val;
                if self.求解喵(row, col + 1) {
                    return true;
                }
                self.board[row][col] = zero;
            }
            false
        })
    }

    pub fn 求解(&mut self) {
        self.feasible = self.求解喵(0, 0);
    }
}

impl<'a, T: Element> Display for 数独<'a, T>
where
    <T as FromStr>::Err: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, row) in self.board.iter().enumerate() {
            for v in row {
                write!(f, "{} ", v)?;
            }
            if id != N - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

// 运筹学

fn main() {
    let mut mat = 数独::<u32>::construct_board_from_stdin();
    let mut 数独 = 数独::new(&mut mat);
    数独.求解();
    print!("{}", 数独);
}

// 送 leetcode 喽

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut 数独 = 数独::new(board);
        数独.求解();
    }
}
