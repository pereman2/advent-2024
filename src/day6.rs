use std::{hash::Hash, hash::Hasher};

use crate::{in_bounds, Parser};

pub fn day_6_1() {
    let mut parser = Parser::new("input6.txt");
    let mut map = Vec::with_capacity(100);

    let mut row = 0;
    let mut cr = 0;
    let mut cc = 0;
    loop {
        if parser.is_eof() {
            break;
        }
        let mut r = Vec::with_capacity(100);
        let mut col = 0;
        loop {
            let mut v = parser.advance(1);
            let line = v == b'\n';
            if line {
                row += 1;
                break;
            }

            if v == b'^' {
                cr = row;
                cc = col;
                v = b'.';
            }
            col += 1;
            r.push(v);
        }
        map.push(r);
    }

    let n = map.len();
    let m = map[0].len();

    let dxs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut current_dx = 0; // start up
    let mut current = (cr, cc);
    let mut res = 0;
    loop {
        if map[current.0 as usize][current.1 as usize] == b'.' {
            map[current.0 as usize][current.1 as usize] = b'X';
            res += 1;
        }
        let mut next = current;
        next.0 += dxs[current_dx].0;
        next.1 += dxs[current_dx].1;
        if !in_bounds(next.0, next.1, n, m) {
            break;
        } else {
            if map[next.0 as usize][next.1 as usize] == b'#' {
                current_dx = (current_dx + 1) % dxs.len();
            } else {
                current = next;
            }
        }
    }
    dbg!(res);
}

pub fn day_6_2() {
    let mut parser = Parser::new("input6.txt");
    let mut map = Vec::with_capacity(100);

    let mut row = 0;
    let mut cr = 0;
    let mut cc = 0;
    let mut obstructions = Vec::new();
    loop {
        if parser.is_eof() {
            break;
        }
        let mut r = Vec::with_capacity(100);
        let mut col = 0;
        loop {
            let mut v = parser.advance(1);
            let line = v == b'\n';
            if line {
                row += 1;
                break;
            }

            if v == b'.' {
                obstructions.push((row, col));
            }
            if v == b'^' {
                cr = row;
                cc = col;
                v = b'.';
            }
            col += 1;
            r.push(v);
        }
        map.push(r);
    }

    let n = map.len() as i64;
    let m = map[0].len() as i64;

    let dxs = [(-1_i64, 0_i64), (0, 1), (1, 0), (0, -1)];
    let mut res = 0;

    let mut seen = vec![0; 130 * 130 * 4];
    for obstruction in &obstructions {
        let mut current = (cr as i64, cc as i64);
        let mut current_dx = 0 as usize; // start up
        seen.fill(0);
        loop {
            let k = current_dx as i64 * (n * m) + current.1 * m + current.0;
            if seen[k as usize] == 1 {
                // loop found
                res += 1;
                break;
            }
            seen[k as usize] = 1;
            let mut next = current;
            next.0 += dxs[current_dx as usize].0;
            next.1 += dxs[current_dx as usize].1;
            if !in_bounds(next.0, next.1, n as usize, m as usize) {
                break;
            } else {
                if map[next.0 as usize][next.1 as usize] == b'#' || next == *obstruction {
                    current_dx = (current_dx + 1) % dxs.len();
                } else {
                    current = next;
                }
            }
        }
    }
    dbg!(res);
}
