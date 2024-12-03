#![feature(test)]
extern crate test;

use std::fs::File;

use day1::{day_1_1, day_1_2};
use day2::{day_2_1, day_2_2};
use day3::day_3_1;
use day3::day_3_2;
use memmap::Mmap;
use memmap::MmapOptions;
pub mod day1;
pub mod day2;
pub mod day3;

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

pub struct Parser {
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

    pub fn try_parse_int(&mut self) -> Option<i64> {
        let buf = self.current();
        let mut pos = 0;
        loop {
            if !buf[pos].is_ascii_digit() {
                break;
            }
            pos += 1;
        }
        let str = unsafe { std::str::from_utf8_unchecked(&buf[0..pos]) };
        let res = str.parse::<i64>();
        match res {
            Ok(v) => {
                self.pos += pos;
                Some(v)
            }
            Err(_) => None,
        }
    }

    pub fn advance(&mut self, amount: usize) -> u8 {
        let buf = self.current();
        let cur = buf[0];
        self.pos += amount;
        cur
    }

    pub fn jump_after(&mut self, v: u8) {
        loop {
            let buf = self.current();
            if buf[0] == v {
                self.pos += 1;
                break;
            }
            self.pos += 1;
        }
    }

    pub fn peek(&mut self) -> u8 {
        let buf = self.current();
        buf[0]
    }

    pub fn ascii_word(&mut self) -> &'static str {
        let mut pos = 0;
        let buf = self.current();
        loop {
            if buf[pos].is_ascii_alphabetic() || buf[pos] == b'\'' {
                pos += 1;
            } else {
                break;
            }
        }
        let str = unsafe {
            // fuck it we ball
            // Obviously this is unsafe if you don't keep the parser alive but you are stupid if
            // you don't.
            std::mem::transmute::<&str, &'static str>(std::str::from_utf8_unchecked(&buf[0..pos]))
        };
        self.advance(pos);
        str
    }
}

bench_fns!(
    (bench_day_1_1, day_1_1),
    (bench_day_1_2, day_1_2),
    (bench_day_2_1, day_2_1),
    (bench_day_2_2, day_2_2),
    (bench_day_3_1, day_3_1),
    (bench_day_3_2, day_3_2)
);

fn main() {
    day_3_2();
}
