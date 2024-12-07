use crate::Parser;

pub fn calc(values: &[i64], res: i64, expect: i64) -> i64 {
    if values.len() < 1 {
        if res == expect {
            return 1;
        } else {
            return 0;
        }
    }
    let a = calc(&values[1..], res + values[0], expect);
    if a > 0 {
        return a;
    }
    let b = calc(&values[1..], res * values[0], expect);
    // a b c d e
    // a b c d e
    return b;
}

pub fn day_7_1() {
    let mut parser = Parser::new("input7.txt");
    let mut values = Vec::with_capacity(1000);
    let mut res = 0;
    loop {
        if parser.is_eof() {
            break;
        }
        let value = parser.parse_int();
        parser.advance(2);
        values.clear();

        loop {
            let v = parser.parse_int();
            values.push(v);
            let line = parser.advance(1) == b'\n';
            if line {
                break;
            }
        }
        let a = calc(&values.as_slice()[2..], values[0] + values[1], value);
        if a > 0 {
            res += value;
        } else {
            let a = calc(&values.as_slice()[2..], values[0] * values[1], value);
            if a > 0 {
                res += value;
            }
        }
    }
    dbg!(res);
}

fn concat(a: i64, b: i64) -> i64 {
    let mut v = b;
    let mut v2 = 0;
    let mut res = a;
    // println!("{} || {}", res, v);
    let leading_zero = (v % 10) == 0;
    while v > 0 {
        let r = v % 10;
        v /= 10;
        v2 *= 10;
        v2 += r;
    }

    while v2 > 0 {
        let r = v2 % 10;
        v2 /= 10;
        res *= 10;
        res += r;
    }
    if leading_zero {
        res *= 10;
    }
    // println!("={}", res);
    // println!("");
    res
}

pub fn calc2(values: &[i64], res: i64, expect: i64) -> i64 {
    if res > expect {
        return 0;
    }
    if values.is_empty() {
        if res == expect {
            return 1;
        } else {
            return 0;
        }
    }
    let a = calc2(&values[1..], res + values[0], expect);
    if a > 0 {
        return a;
    }
    let b = calc2(&values[1..], res * values[0], expect);
    if b > 0 {
        return b;
    }
    calc2(&values[1..], concat(res, values[0]), expect)
}

pub fn day_7_2() {
    let mut parser = Parser::new("input7.txt");
    let mut values = Vec::with_capacity(1000);
    let mut res = 0;
    loop {
        if parser.is_eof() {
            break;
        }
        let value = parser.parse_int();
        parser.advance(2);
        values.clear();

        loop {
            let v = parser.parse_int();
            values.push(v);
            let line = parser.advance(1) == b'\n';
            if line {
                break;
            }
        }
        let a = calc2(&values.as_slice()[2..], values[0] + values[1], value);
        if a > 0 {
            res += value;
        } else {
            let a = calc2(&values.as_slice()[2..], values[0] * values[1], value);
            if a > 0 {
                res += value;
            } else {
                let a = calc2(&values.as_slice()[2..], concat(values[0], values[1]), value);
                if a > 0 {
                    res += value;
                }
            }
        }
    }
    dbg!(res);
}

enum Op {
    Mul,
    Sum,
    Concat,
}

pub fn day_7_2_combin() {
    let mut parser = Parser::new("input7.txt");
    let mut values = Vec::with_capacity(1000);
    let mut combinations: Vec<Op> = Vec::with_capacity(3.pow(100));
    let mut res = 0;
    loop {
        if parser.is_eof() {
            break;
        }
        let value = parser.parse_int();
        parser.advance(2);
        values.clear();

        loop {
            let v = parser.parse_int();
            values.push(v);
            let line = parser.advance(1) == b'\n';
            if line {
                break;
            }
        }
        // 3 ^ n
        let a = calc2(&values.as_slice()[2..], values[0] + values[1], value);
        if a > 0 {
            res += value;
        } else {
            let a = calc2(&values.as_slice()[2..], values[0] * values[1], value);
            if a > 0 {
                res += value;
            } else {
                let a = calc2(&values.as_slice()[2..], concat(values[0], values[1]), value);
                if a > 0 {
                    res += value;
                }
            }
        }
    }
    dbg!(res);
}
