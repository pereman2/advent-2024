use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::Parser;

pub struct Matrix<T> {
    pub ar: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}
impl<T> Matrix<T>
where
    T: Copy + std::fmt::Debug,
{
    pub fn new(ar: Vec<T>, rows: usize, cols: usize) -> Self {
        Self { ar, rows, cols }
    }

    pub fn get_copy(&self, i: i64, j: i64) -> T {
        self.ar[i as usize * self.cols + j as usize]
    }

    pub fn get_mut(&mut self, i: i64, j: i64) -> &mut T {
        &mut self.ar[i as usize * self.cols + j as usize]
    }

    pub fn in_bounds(&self, a: i64, b: i64) -> bool {
        a >= 0 && a < self.rows as i64 && b >= 0 && b < self.cols as i64
    }

    pub fn show(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let c = self.get_copy(i as i64, j as i64);
                print!("{:?}|", c);
            }
            println!("");
        }
    }
}

pub fn day_10_1() {
    let mut parser = Parser::new("input10.txt");
    let mut ar = Vec::with_capacity(10000);
    let mut start = Vec::with_capacity(1000);
    let mut rows = 0;
    let mut cols = 0;
    let mut colsm = 0;
    while !parser.is_eof() {
        loop {
            let c = parser.advance(1);
            let line = c == b'\n';
            if line {
                colsm = colsm.max(cols);
                cols = 0;
                rows += 1;
                break;
            }
            let c = c - b'0';
            if c == 0 {
                start.push((rows, cols));
            }
            ar.push(c);
            cols += 1;
        }
    }

    let mut res = 0;
    let mut m = Matrix::new(ar, rows, colsm);
    let mut stack = Vec::with_capacity(1000);
    let mut trail_heads = HashSet::with_capacity(1000);
    let dxs = [(0_i64, 1_i64), (0, -1), (1, 0), (-1, 0)];
    while !start.is_empty() {
        let current_start = start.pop().unwrap();
        stack.clear();
        trail_heads.clear();

        stack.push(current_start);
        let mut heads = 0;
        while !stack.is_empty() {
            let (i, j) = stack.pop().unwrap();
            let c = m.get_copy(i as i64, j as i64);
            if trail_heads.contains(&(i, j)) {
                continue;
            }
            trail_heads.insert((i, j));
            if c == 9 {
                heads += 1;

                continue;
            }
            for dx in &dxs {
                let new_i = i as i64 + dx.0;
                let new_j = j as i64 + dx.1;
                if !m.in_bounds(new_i, new_j) {
                    continue;
                }
                let other = m.get_copy(new_i as i64, new_j as i64);
                if other == c + 1 {
                    stack.push((new_i as usize, new_j as usize));
                }
            }
        }
        res += heads;
    }

    dbg!(res);
}

pub fn day_10_2() {
    let mut parser = Parser::new("input10.txt");
    let mut ar = Vec::with_capacity(10000);
    let mut start = Vec::with_capacity(1000);
    let mut rows = 0;
    let mut cols = 0;
    let mut colsm = 0;
    while !parser.is_eof() {
        loop {
            let c = parser.advance(1);
            let line = c == b'\n';
            if line {
                colsm = colsm.max(cols);
                cols = 0;
                rows += 1;
                break;
            }
            let c = c - b'0';
            if c == 0 {
                start.push((rows, cols));
            }
            ar.push(c);
            cols += 1;
        }
    }

    let mut res = 0;
    let mut m = Matrix::new(ar, rows, colsm);
    let mut stack = Vec::with_capacity(1000);
    let dxs = [(0_i64, 1_i64), (0, -1), (1, 0), (-1, 0)];
    while !start.is_empty() {
        let current_start = start.pop().unwrap();
        stack.clear();

        stack.push(current_start);
        let mut heads = 0;
        while !stack.is_empty() {
            let (i, j) = stack.pop().unwrap();
            let c = m.get_copy(i as i64, j as i64);
            if c == 9 {
                heads += 1;
                continue;
            }
            for dx in &dxs {
                let new_i = i as i64 + dx.0;
                let new_j = j as i64 + dx.1;
                if !m.in_bounds(new_i, new_j) {
                    continue;
                }
                let other = m.get_copy(new_i as i64, new_j as i64);
                if other == c + 1 {
                    stack.push((new_i as usize, new_j as usize));
                }
            }
        }
        res += heads;
    }

    dbg!(res);
}
