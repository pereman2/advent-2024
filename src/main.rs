#![feature(test)]
extern crate test;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use std::fs::File;

use day1::*;
use day2::*;
use day3::*;
use day3::*;
use day4::*;
use day5::*;
use day6::*;
use day7::*;
use day8::*;
use day9::*;
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

pub struct Parser {
    file: File,
    mmap: Mmap,
    pos: usize,
    len: usize,
    mmap_buf: &'static [u8],
}

impl Parser {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
        let len = mmap.len();
        let mmap_buf = unsafe { std::mem::transmute::<&[u8], &'static [u8]>(mmap.as_ref()) };

        Self {
            file,
            mmap,
            pos: 0,
            len,
            mmap_buf,
        }
    }

    pub fn is_eof(&mut self) -> bool {
        self.pos >= self.len
    }

    pub fn current(&mut self) -> &[u8] {
        let buf = self.mmap_buf;
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
    (bench_day_3_2, day_3_2),
    (bench_day_4_1, day_4_1),
    (bench_day_4_2, day_4_2),
    (bench_day_5_1, day_5_1),
    (bench_day_5_2, day_5_2),
    (bench_day_6_1, day_6_1),
    (bench_day_6_2, day_6_2),
    (bench_day_7_1, day_7_1),
    (bench_day_7_2, day_7_2),
    (bench_day_7_2_stack, day_7_2_stack),
    (bench_day_8_1, day_8_1),
    (bench_day_8_2, day_8_2),
    (bench_day_9_1, day_9_1),
    (bench_day_9_2, day_9_2)
);

fn main() {
    day_9_2();
}
