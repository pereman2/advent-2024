use crate::Parser;

pub fn day_9_1() {
    let mut parser = Parser::new("input9.txt");
    let mut b = true;
    let mut blocks = Vec::with_capacity(10000);
    let mut free = Vec::with_capacity(10000);
    let mut vv = vec!['.'; 100];
    let mut pos: usize = 0;
    loop {
        if parser.is_eof() {
            break;
        }
        let n = parser.advance(1);
        if n == b'\n' {
            break;
        }
        let n = n - b'0';
        if b {
            blocks.push((n, pos));
        } else {
            free.push((n, pos));
        }
        pos += n as usize;
        b = !b;
    }

    let mut freep: usize = 0;
    let mut blockp: usize = blocks.len() - 1;

    let mut res = 0;
    let (mut current_free, mut current_free_pos) = free[freep];
    let (mut current_block, mut current_block_pos) = blocks[blockp];
    while freep < free.len() {
        if current_free_pos >= current_block_pos {
            break;
        }
        // take as much as we can from current block and put as much as we can in this free block
        let to_set = current_free.min(current_block);
        for i in current_free_pos..current_free_pos + to_set as usize {
            res += i * blockp;
        }
        current_free -= to_set;
        current_block -= to_set;
        current_free_pos += to_set as usize;
        current_block_pos -= to_set as usize;

        if current_free == 0 {
            freep += 1;
            if freep >= free.len() {
                break;
            }

            (current_free, current_free_pos) = free[freep];
        }
        if current_block == 0 {
            if blockp == 0 {
                break;
            }
            blockp -= 1;
            (current_block, current_block_pos) = blocks[blockp];
            current_block_pos += current_block as usize;
        }
    }
    for (i, (block, pos)) in blocks.iter().enumerate() {
        for j in *pos..*pos + *block as usize {
            if j >= current_block_pos {
                break;
            }
            res += j * i;
        }
    }
    dbg!(res);
}

pub fn day_9_2() {
    let mut parser = Parser::new("input9.txt");
    let mut b = true;
    let mut blocks = Vec::with_capacity(10000);
    let mut free = Vec::with_capacity(10000);
    let mut id = 0;
    let mut pos: usize = 0;
    loop {
        if parser.is_eof() {
            break;
        }
        let n = parser.advance(1);
        if n == b'\n' {
            break;
        }
        let n = n - b'0';
        if b {
            blocks.push((n, pos, false, id));
            id += 1;
        } else {
            free.push((n, pos));
        }
        pos += n as usize;
        b = !b;
    }

    let mut new_free = Vec::with_capacity(10000);
    for block in blocks.iter_mut().rev() {
        for (c, pos) in free.iter_mut() {
            if block.1 <= *pos {
                break;
            }
            if block.0 <= *c {
                new_free.push((block.0, *pos, block.3));
                block.2 = true;
                *c -= block.0;
                *pos += block.0 as usize;
                break;
            }
        }
    }
    let mut res = 0;
    for block in blocks {
        if !block.2 {
            for i in block.1..block.1 + block.0 as usize {
                res += i * block.3;
            }
        }
    }
    for free in new_free {
        for i in free.1..free.1 + free.0 as usize {
            res += i * free.2;
        }
    }
    dbg!(res);
}
