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

fn concat(x: i64, y: i64) -> i64 {
    let m = ((y >= 10) as i64 * 90 + (y >= 100) as i64 * 900 + (y >= 1000) as i64 * 9000) + 10;
    m * x + y
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

pub fn day_7_2_stack() {
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
        let mut stack = Vec::with_capacity(10000);
        stack.push((values[0] + values[1], &values.as_slice()[2..]));
        stack.push((values[0] * values[1], &values.as_slice()[2..]));
        stack.push((concat(values[0], values[1]), &values.as_slice()[2..]));
        while let Some((acc, values)) = stack.pop() {
            if values.is_empty() && acc == value {
                res += value;
                break;
            }
            if values.is_empty() {
                continue;
            }
            if acc > value {
                continue;
            }

            stack.push((acc + values[0], &values[1..]));
            stack.push((acc * values[0], &values[1..]));
            stack.push((concat(acc, values[0]), &values[1..]));
        }
    }
    dbg!(res);
}
