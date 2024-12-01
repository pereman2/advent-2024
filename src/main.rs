#![feature(test)]
extern crate test;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use memmap::Mmap;
use memmap::MmapOptions;

macro_rules! bench_fns {
    ($(($name:ident, $func_name:ident)),*) => {
        $(
            #[bench]
            fn $name(b: &mut test::Bencher) {
                b.iter(|| $func_name());
            }
        )*
    };
}

struct Parser {
    file: File,
    mmap: Mmap,
    pos: usize,
}

impl Parser {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

        Self { file, mmap, pos: 0 }
    }

    pub fn is_eof(&mut self) -> bool {
        self.pos >= self.mmap.len()
    }

    pub fn current(&mut self) -> &[u8] {
        let buf = self.mmap.as_ref();
        &buf[self.pos..]
    }

    pub fn parse_int(&mut self) -> i64 {
        let buf = self.current();
        let mut pos = 0;
        loop {
            if !buf[pos].is_ascii_digit() {
                break;
            }
            pos += 1;
        }
        let str = unsafe { std::str::from_utf8_unchecked(&buf[0..pos]) };
        let res = str.parse::<i64>().unwrap();
        self.pos += pos;
        res
    }

    pub fn advance(&mut self, amount: usize) {
        self.pos += amount;
    }
}

fn day_1_1() {
    let mut parser = Parser::new("input1.txt");
    let mut vv = Vec::new();
    let mut ww = Vec::new();
    loop {
        if parser.is_eof() {
            break;
        }

        let a = parser.parse_int();
        parser.advance(3);
        let b = parser.parse_int();
        parser.advance(1);
        vv.push(a);
        ww.push(b);
    }

    vv.sort();
    ww.sort();

    let mut res = 0;
    for i in 0..vv.len() {
        res += vv[i].abs_diff(ww[i]);
    }
    dbg!(res);
}

fn day_1_2() {
    let mut parser = Parser::new("input1.txt");
    let mut vv = HashSet::with_capacity(5000);
    // The biggest number doesn't exceed 100K so we can improve the hashmap to be a linear vector
    let mut ww = vec![0; 100000];
    loop {
        if parser.is_eof() {
            break;
        }

        let a = parser.parse_int();
        parser.advance(3);
        let b = parser.parse_int();
        parser.advance(1);
        vv.insert(a);
        let v = ww.get_mut(b as usize).unwrap();
        *v += 1;
    }

    let mut res = 0;
    for i in vv {
        res += i * ww[i as usize];
    }
    assert_eq!(res, 21070419);
    dbg!(res);
}

bench_fns!((bench_day_1_1, day_1_1), (bench_day_1_2, day_1_2));

fn main() {
    day_1_2();
}
