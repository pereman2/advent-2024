#![feature(test)]
extern crate test;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

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
    file: BufReader<File>,
}

impl Parser {
    pub fn new(path: &str) -> Self {
        Self {
            file: BufReader::new(File::open(path).unwrap()),
        }
    }

    pub fn next_line<'a>(&mut self, buf: &'a mut Vec<u8>) -> (usize, &'a str) {
        buf.clear();
        let res = self.file.read_until('\n' as u8, buf).unwrap();
        let line = unsafe {
            let len = if res == 0 { 0 } else { buf.len() - 1 };
            std::str::from_utf8_unchecked(&buf.as_slice()[0..len])
        };
        (res, line)
    }
}

fn day_1_1() {
    let mut parser = Parser::new("input1.txt");
    let mut buf = Vec::with_capacity(4096);
    let mut vv = Vec::new();
    let mut ww = Vec::new();
    loop {
        let (res, line) = parser.next_line(&mut buf);
        if res == 0 {
            break;
        }

        let mut spaces = line.split(" ");
        let a = spaces.next().unwrap().parse::<i64>().unwrap();
        let b = spaces.next().unwrap().parse::<i64>().unwrap();
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
    let mut buf = Vec::with_capacity(4096);
    let mut vv = HashSet::with_capacity(5000);
    // The biggest number doesn't exceed 100K so we can improve the hashmap to be a linear vector
    let mut ww = vec![0; 100000];
    loop {
        let (res, line) = parser.next_line(&mut buf);
        if res == 0 {
            break;
        }

        let mut spaces = line.split(" ");
        let a = spaces.next().unwrap().parse::<i64>().unwrap();
        let b = spaces.next().unwrap().parse::<i64>().unwrap();
        vv.insert(a);
        let v = ww.get_mut(b as usize).unwrap();
        *v += 1;
    }

    let mut res = 0;
    for i in vv {
        res += i * ww[i as usize];
    }
    dbg!(res);
}

bench_fns!((bench_day_1_1, day_1_1), (bench_day_1_2, day_1_2));

fn main() {
    day_1_2();
}
