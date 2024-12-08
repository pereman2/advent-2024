// 4,5
// 6,6
//
// 3,6
// 4,8
//
// 2 / 1
// y-8 = 2 (x-4)
// y = 2x -8 +8
// y = 2x
//
//
// 6,7
// 9,9
//
// m = 2 / 3
//
// y - 9 = 2/3(x - 9)
// y - 9 = 2/3(x - 9)
// y = 2/3x + 3
// y(3) = 2 + 3 = 5
//
//    a
// y= - + c
//    b
//
//

use crate::{in_bounds, Parser};

pub fn day_8_1() {
    let mut parser = Parser::new("input8.txt");
    let mut antenas: Vec<Vec<(i64, i64)>> = vec![Vec::new(); 512];

    let mut x = 0;
    let mut n = 0;
    let mut m = 0;

    loop {
        if parser.is_eof() {
            break;
        }
        let mut y = 0;
        loop {
            let v = parser.advance(1);
            let line = v == b'\n';
            if line {
                m = y;
                break;
            }
            if v != b'.' {
                antenas[v as usize].push((x, y));
            }
            y += 1;
        }
        x += 1;
    }
    n = x;

    let mut antinodes = vec![false; n as usize * m as usize];
    for antena in &antenas {
        if antena.is_empty() {
            continue;
        }
        for (x, y) in antena {
            for (x2, y2) in antena {
                if x == x2 && y == y2 {
                    continue;
                }
                let a = y2 - y;
                let b = x2 - x;
                // (1, 8) (3, 7)
                // 7 - 8
                // -1 / 2
                // y - 7 = -1/2x + 3/2
                //
                // let's not remove fractions
                // y - y2 = (a/b)(x - x2)
                // y  = (a/b)(x - x2) + y2
                // y  = (a/b)(x - x2) + (y2*b)/b
                // y  = (1/b)(ax - ax2) + (y2*b)/b
                // y  = (1/b)(ax - ax2 +(y2*b))
                // y  = (1/b)(ax - c + d)
                let c = a * x2;
                let d = y2 * b;
                debug_assert!(((a * x) - c + d) % b == 0);

                let mut rx = x2.max(x) + 1;
                while ((a * rx) - c + d) % b != 0 {
                    rx += 1;
                }
                let ry = ((a * rx) - c + d) / b;
                if in_bounds(rx, ry, n as usize, m as usize) {
                    antinodes[(ry * m + rx) as usize] = true;
                }

                let mut rx = x2.min(x) - 1;
                while ((a * rx) - c + d) % b != 0 {
                    rx -= 1;
                }
                let ry = ((a * rx) - c + d) / b;
                if in_bounds(rx, ry, n as usize, m as usize) {
                    antinodes[(ry * m + rx) as usize] = true;
                }
            }
        }
    }

    let res = antinodes.iter().fold(0, |acc, v| acc + *v as usize);
    dbg!(res);
}

pub fn day_8_2() {
    let mut parser = Parser::new("input8.txt");
    let mut antenas: Vec<Vec<(i64, i64)>> = vec![Vec::new(); 512];

    let mut x = 0;
    let mut n = 0;
    let mut m = 0;

    loop {
        if parser.is_eof() {
            break;
        }
        let mut y = 0;
        loop {
            let v = parser.advance(1);
            let line = v == b'\n';
            if line {
                m = y;
                break;
            }
            if v != b'.' {
                antenas[v as usize].push((x, y));
            }
            y += 1;
        }
        x += 1;
    }
    n = x;

    let mut antinodes = vec![false; n as usize * m as usize];
    for antena in &antenas {
        if antena.is_empty() {
            continue;
        }
        for (x, y) in antena {
            for (x2, y2) in antena {
                if x == x2 && y == y2 {
                    continue;
                }
                let a = *y2 - y;
                let b = *x2 - x;
                let c = a * *x2;
                let d = *y2 * b;
                debug_assert!(((a * x) - c + d) % b == 0);

                for rx in 0..n {
                    if ((a * rx) - c + d) % b != 0 {
                        continue;
                    }
                    let ry = ((a * rx) - c + d) / b;
                    if !in_bounds(rx, ry, n as usize, m as usize) {
                        continue;
                    }
                    antinodes[(ry * m + rx) as usize] = true;
                }
            }
        }
    }

    let res = antinodes.iter().fold(0, |acc, v| acc + *v as usize);
    dbg!(res);
}
