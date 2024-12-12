use std::collections::HashSet;

use crate::{Matrix, Parser, DXS};

pub fn day_12_1() {
    let mut parser = Parser::new("input12.txt");
    let mut values = Vec::with_capacity(10000);
    let mut rows = 0;
    let mut cols = 0;
    let mut colsn = 0;
    while !parser.is_eof() {
        loop {
            let c = parser.advance(1);
            if c == b'\n' {
                colsn = colsn.max(cols);
                cols = 0;
                rows += 1;
                break;
            }
            cols += 1;
            values.push(c);
        }
    }
    let mut res = 0;
    let mut seen = HashSet::new();
    let m = Matrix::new(values, rows, colsn);
    for i in 0..rows {
        for j in 0..colsn {
            let key = (i as i64, j as i64);
            if seen.contains(&key) {
                continue;
            }
            let mut hor = Matrix::new(vec![0; (rows + 1) * colsn], rows + 1, colsn);
            let mut vert = Matrix::new(vec![0; rows * (colsn + 1)], rows, colsn + 1);
            let mut stack = Vec::new();

            let c = m.get_copy(i as i64, j as i64);
            // build edges matrix
            stack.push((i as i64, j as i64));
            let mut nodes = 0;
            while !stack.is_empty() {
                let current = stack.pop().unwrap();
                if seen.contains(&current) {
                    continue;
                }
                nodes += 1;
                seen.insert(current);
                for dx in DXS {
                    let next = (current.0 + dx.0, current.1 + dx.1);
                    if m.in_bounds(next.0, next.1)
                        && m.get_copy(next.0, next.1) == m.get_copy(current.0, current.1)
                    {
                        stack.push(next);
                    } else {
                        match dx {
                            // horizontal
                            (-1, 0) => {
                                let v = hor.get_mut(current.0, current.1);
                                *v = 1;
                            }
                            (1, 0) => {
                                let v = hor.get_mut(current.0 + 1, current.1);
                                *v = 1;
                            }
                            // vertical
                            (0, 1) => {
                                let h = vert.get_mut(current.0, current.1 + 1);
                                *h = 1;
                            }
                            (0, -1) => {
                                let h = vert.get_mut(current.0, current.1);
                                *h = 1;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
            let vc = vert.ar.iter().fold(0, |acc, v| acc + *v as usize);
            let hc = hor.ar.iter().fold(0, |acc, v| acc + *v as usize);
            let sol = nodes * (hc + vc);
            res += sol;
        }
    }
    dbg!(res);
}

pub fn day_12_2() {
    let mut parser = Parser::new("input12.txt");
    let mut values = Vec::with_capacity(10000);
    let mut rows = 0;
    let mut cols = 0;
    let mut colsn = 0;
    while !parser.is_eof() {
        loop {
            let c = parser.advance(1);
            if c == b'\n' {
                colsn = colsn.max(cols);
                cols = 0;
                rows += 1;
                break;
            }
            cols += 1;
            values.push(c);
        }
    }
    let mut res = 0;
    let mut seen = HashSet::new();
    // dbg!(rows, colsn);
    let m = Matrix::new(values, rows, colsn);
    for i in 0..rows {
        for j in 0..colsn {
            let key = (i as i64, j as i64);
            if seen.contains(&key) {
                continue;
            }
            let mut hor = Matrix::new(vec![(false, (0, 0)); (rows + 1) * colsn], rows + 1, colsn);
            let mut vert = Matrix::new(vec![(false, (0, 0)); rows * (colsn + 1)], rows, colsn + 1);
            let mut stack = Vec::new();

            let c = m.get_copy(i as i64, j as i64);
            // build edges matrix
            stack.push((i as i64, j as i64));
            let mut nodes = 0;
            while !stack.is_empty() {
                let current = stack.pop().unwrap();
                if seen.contains(&current) {
                    continue;
                }
                nodes += 1;
                seen.insert(current);
                // dbg!(current);
                for dx in DXS {
                    let next = (current.0 + dx.0, current.1 + dx.1);
                    // dbg!(next);
                    if m.in_bounds(next.0, next.1)
                        && m.get_copy(next.0, next.1) == m.get_copy(current.0, current.1)
                    {
                        stack.push(next);
                    } else {
                        match dx {
                            // horizontal
                            (-1, 0) => {
                                let v = hor.get_mut(current.0, current.1);
                                *v = (true, current);
                            }
                            (1, 0) => {
                                let v = hor.get_mut(current.0 + 1, current.1);
                                *v = (true, current);
                            }
                            // vertical
                            (0, 1) => {
                                let h = vert.get_mut(current.0, current.1 + 1);
                                *h = (true, current);
                            }
                            (0, -1) => {
                                let h = vert.get_mut(current.0, current.1);
                                *h = (true, current);
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }

            // check verticallly
            let mut vsides = 0;
            for y in 0..vert.cols {
                let mut in_line = false;
                for x in 0..vert.rows {
                    match (in_line, vert.get_copy(x as i64, y as i64)) {
                        (true, (true, (i, j))) => {
                            // check if previous is connected to this one
                            let (b, previous) = vert.get_copy(x as i64 - 1, y as i64);
                            assert!(b);
                            if m.get_copy(i - 1, j) != m.get_copy(previous.0, previous.1) {
                                // this is the case were we have a line that goes across different
                                // sides but looks like a single one
                                dbg!((i, j), previous);
                                vsides += 1;
                            }
                        }
                        (true, (false, _)) => {
                            // we are leaving a line so we finished a side
                            in_line = false;
                            vsides += 1;
                        }
                        (false, (true, _)) => {
                            in_line = true;
                        }
                        (false, (false, _)) => {}
                    }
                }
                if in_line {
                    // we are leaving a line so we finished a side
                    vsides += 1;
                }
            }
            let mut hsides = 0;
            // check horizontally
            for x in 0..hor.rows {
                let mut in_line = false;
                for y in 0..hor.cols {
                    match (in_line, hor.get_copy(x as i64, y as i64)) {
                        (true, (true, (i, j))) => {
                            // check if previous is connected to this one
                            let (_, previous) = hor.get_copy(x as i64, y as i64 - 1);
                            if m.get_copy(i, j - 1) != m.get_copy(previous.0, previous.1) {
                                // this is the case were we have a line that goes across different
                                // sides but looks like a single one
                                hsides += 1;
                            }
                        }
                        (true, (false, _)) => {
                            // we are leaving a line so we finished a side
                            in_line = false;
                            hsides += 1;
                        }
                        (false, (true, _)) => {
                            in_line = true;
                        }
                        (false, (false, _)) => {}
                    }
                }
                if in_line {
                    // we are leaving a line so we finished a side
                    hsides += 1;
                }
            }
            let sides = hsides + vsides;
            let sol = nodes * sides;
            res += sol;
        }
    }
    dbg!(res);
}
