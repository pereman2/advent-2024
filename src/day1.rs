use std::collections::HashSet;

use crate::Parser;

pub fn day_1_1() {
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

pub fn day_1_2() {
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
