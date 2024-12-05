use std::collections::HashMap;

use crate::Parser;

#[derive(Debug)]
struct Node {
    dependent: Vec<i64>,
}

pub fn day_5_1() {
    let mut parser = Parser::new("input5.txt");
    let mut rules: HashMap<i64, Node> = HashMap::new();

    loop {
        if parser.peek() == b'\n' {
            parser.advance(1);
            break;
        }
        let a = parser.parse_int();
        parser.advance(1);
        let b = parser.parse_int();
        parser.advance(1);

        if let Some(node) = rules.get_mut(&a) {
            node.dependent.push(b);
        } else {
            let mut node = Node {
                dependent: Vec::new(),
            };
            node.dependent.push(b);
            rules.insert(a, node);
        }

        if !rules.contains_key(&b) {
            let node = Node {
                dependent: Vec::new(),
            };
            rules.insert(b, node);
        }
    }

    let mut values = Vec::new();
    let mut res = 0;
    loop {
        if parser.is_eof() {
            break;
        }
        let mut ok = true;

        values.clear();
        let mut updated = HashMap::new();
        loop {
            let v = parser.parse_int();
            let line = parser.advance(1) == b'\n';
            if !ok {
                if line {
                    break;
                }
                continue;
            }

            values.push(v);
            if !updated.contains_key(&v) {
                updated.insert(v, values.len());
            }

            if let Some(node) = rules.get(&v) {
                let others = node.dependent.clone();
                for node in others {
                    if let Some(up) = updated.get(&node) {
                        if *up < values.len() {
                            ok = false;
                        }
                    }
                }
            }
            if line {
                break;
            }
        }
        let mid = values.len() / 2;
        let mid = values[mid];
        if ok {
            res += mid;
        }
    }
    dbg!(res);
}

pub fn day_5_2() {
    let mut parser = Parser::new("input5.txt");
    let mut rules: HashMap<i64, Node> = HashMap::new();

    loop {
        if parser.peek() == b'\n' {
            parser.advance(1);
            break;
        }
        let a = parser.parse_int();
        parser.advance(1);
        let b = parser.parse_int();
        parser.advance(1);

        if let Some(node) = rules.get_mut(&a) {
            node.dependent.push(b);
        } else {
            let mut node = Node {
                dependent: Vec::new(),
            };
            node.dependent.push(b);
            rules.insert(a, node);
        }

        if !rules.contains_key(&b) {
            let node = Node {
                dependent: Vec::new(),
            };
            rules.insert(b, node);
        }
    }

    let mut values = Vec::new();
    let mut res = 0;
    loop {
        if parser.is_eof() {
            break;
        }

        values.clear();
        loop {
            let v = parser.parse_int();
            let line = parser.advance(1) == b'\n';
            values.push(v);
            if line {
                break;
            }
        }

        let mut tries = 0;
        let mut start_at = 0;
        loop {
            let mut ok = true;
            let mut move_from = 0;
            let mut move_to = 0;
            let mut move_v = 0;
            let mut updated = HashMap::new();
            for (i, v) in values.iter().enumerate().take(start_at) {
                if !updated.contains_key(v) {
                    updated.insert(*v, i);
                }
            }
            for (i, v) in values.iter().enumerate().skip(start_at) {
                if !updated.contains_key(v) {
                    updated.insert(*v, i);
                }
                if let Some(node) = rules.get(v) {
                    let others = node.dependent.clone();
                    let mut min = i;
                    for node in others {
                        if let Some(up) = updated.get(&node) {
                            min = min.min(*up);
                        }
                    }
                    if min < i {
                        // okay we need to move back
                        ok = false;
                        move_to = min;
                        move_from = i;
                        move_v = *v;
                        // we can prune the search by starting from the place where we moved
                        start_at = i;
                        break;
                    }
                }
            }
            tries += 1;
            if !ok {
                values.remove(move_from);
                values.insert(move_to, move_v);
            }
            if ok {
                if tries > 1 {
                    let mid = values.len() / 2;
                    let mid = values[mid];
                    res += mid;
                }
                break;
            }
        }
    }
    assert_eq!(res, 4598);
    dbg!(res);
}
