use crate::Parser;

fn safe_diff(a: i64, b: i64, ascending: bool) -> bool {
    let diff = a.abs_diff(b);
    if ascending {
        if a >= b {
            return false;
        }
    } else {
        if a <= b {
            return false;
        }
    }
    diff > 0 && diff <= 3
}

pub fn day_2_1() {
    let mut parser = Parser::new("input2.txt");
    let mut safe_count = 0;
    loop {
        if parser.is_eof() {
            break;
        }
        let mut safe = true;

        let mut prev = parser.parse_int();
        parser.advance(1);
        let current = parser.parse_int();
        parser.advance(1);
        let ascending = prev < current;

        if !safe_diff(prev, current, ascending) {
            parser.jump_after(b'\n');
            continue;
        }
        prev = current;

        loop {
            let current = parser.parse_int();
            if !safe_diff(prev, current, ascending) {
                safe = false;
                parser.jump_after(b'\n');
                break;
            }
            prev = current;
            let c = parser.advance(1);
            if c == b'\n' {
                break;
            }
        }
        safe_count += safe as i32;
    }

    dbg!(safe_count);
}

pub fn day_2_2() {
    let mut parser = Parser::new("input2.txt");
    let mut safe_count = 0;
    let mut values = Vec::with_capacity(100);
    'outer: loop {
        if parser.is_eof() {
            break;
        }
        let mut safe = true;
        let mut tries = 1;
        values.clear();

        loop {
            let current = parser.parse_int();
            values.push(current);
            let c = parser.advance(1);
            if c == b'\n' {
                break;
            }
        }

        let mut tries = 0;
        let mut prev = values[2];
        let ascending = values[1] < values[2];
        let mut safe = true;
        if safe_diff(values[1], values[2], ascending) {
            for i in values.iter().skip(3) {
                let current = *i;
                if !safe_diff(prev, current, ascending) {
                    safe = false;
                    break;
                }
                prev = current;
            }
            if safe {
                safe_count += 1;
                continue;
            }
        }
        // remove second
        let mut prev = values[2];
        let ascending = values[0] < values[2];
        if safe_diff(values[0], values[2], ascending) {
            let mut safe = true;
            for i in values.iter().skip(3) {
                let current = *i;
                if !safe_diff(prev, current, ascending) {
                    safe = false;
                    break;
                }
                prev = current;
            }
            if safe {
                safe_count += 1;
                continue;
            }
        }
        // remove third
        let mut prev = values[1];
        let ascending = values[0] < values[1];
        let mut safe = true;
        if safe_diff(values[0], values[1], ascending) {
            for i in values.iter().skip(3) {
                let current = *i;
                if !safe_diff(prev, current, ascending) {
                    safe = false;
                    break;
                }
                prev = current;
            }
            if safe {
                safe_count += 1;
                continue;
            }
        }

        let a = values[0];
        let b = values[1];
        let c = values[2];

        let asa = a < b;
        let asb = b < c;
        // TODO: find valleys and peaks and remove one of them
        if safe_diff(a, b, a < b) && safe_diff(b, c, b < c) && asa == asb {
            for r in 3..values.len() {
                let mut prev = values[2];
                let ascending = values[0] < values[1];
                let mut safe = true;
                for i in 3..values.len() {
                    if i == r {
                        continue;
                    }
                    let current = values[i];
                    if !safe_diff(prev, current, ascending) {
                        safe = false;
                        break;
                    }
                    prev = current;
                }
                if safe {
                    safe_count += 1;
                    continue 'outer;
                }
            }
        }
    }

    assert_eq!(safe_count, 553);

    dbg!(safe_count);
}
