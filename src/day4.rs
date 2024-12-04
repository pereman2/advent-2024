use crate::Parser;

pub fn day_4_1() {
    let mut parser = Parser::new("input4.txt");
    let mut m = Vec::with_capacity(1000);
    let mut xs = Vec::with_capacity(1000);
    let mut row = 0;
    let mut col = 0;
    loop {
        if parser.is_eof() {
            break;
        }
        let mut values = Vec::with_capacity(1000);

        loop {
            let v = parser.advance(1);
            if v == b'\n' {
                row += 1;
                col = 0;
                m.push(values);
                break;
            }

            if v == b'X' {
                xs.push((row, col));
            }
            col += 1;
            values.push(v);
        }
    }
    let dxs = [
        (0_i64, 1_i64),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let word = b"XMAS";
    let mut res = 0;
    for (r, c) in xs {
        for dx in &dxs {
            let mut i = r as i64;
            let mut j = c as i64;
            assert!(m[r][c] == b'X');
            let mut good = true;
            for pos in 1..4 {
                i += dx.0;
                j += dx.1;
                if i < 0
                    || i >= m.len() as i64
                    || j < 0
                    || j >= m[0].len() as i64
                    || word[pos] != m[i as usize][j as usize]
                {
                    good = false;
                    break;
                }
            }
            res += good as usize;
        }
    }
    dbg!(res);
}

fn in_bounds(a: i64, b: i64, n: usize, m: usize) -> bool {
    a >= 0 && a < n as i64 && b >= 0 && b < m as i64
}

pub fn day_4_2() {
    let mut parser = Parser::new("input4.txt");
    let mut m = Vec::with_capacity(1000);
    let mut xs = Vec::with_capacity(1000);
    let mut row = 0;
    let mut col = 0;
    loop {
        if parser.is_eof() {
            break;
        }
        let mut values = Vec::with_capacity(1000);

        loop {
            let v = parser.advance(1);
            if v == b'\n' {
                row += 1;
                col = 0;
                m.push(values);
                break;
            }

            if v == b'A' {
                xs.push((row, col));
            }
            col += 1;
            values.push(v);
        }
    }
    let start_pos = [(1_i64, 1_i64), (1, -1), (-1, 1), (-1, -1)];
    let rows = m.len();
    let cols = m[0].len();

    let word = b"MAS";
    let mut res = 0;
    for (r, c) in xs {
        let mut count = 0_usize;

        for start in &start_pos {
            let dx = (-start.0, -start.1);
            let mut start = (r + start.0, c + start.1);
            let mut good = true;
            for pos in 0..3 {
                if !in_bounds(start.0, start.1, rows, cols)
                    || m[start.0 as usize][start.1 as usize] != word[pos as usize]
                {
                    good = false;
                    break;
                }
                start.0 += dx.0;
                start.1 += dx.1;
            }
            count += good as usize;
        }

        res += (count == 2) as usize;
    }
    dbg!(res);
}
