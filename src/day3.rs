use crate::Parser;

pub fn skip_bullshit(parser: &mut Parser) {
    loop {
        if parser.is_eof() {
            break;
        }
        let v = parser.peek();
        if v.is_ascii_alphabetic() {
            break;
        }
        parser.advance(1);
    }
}

pub fn day_3_1() {
    let mut parser = Parser::new("input3.txt");

    let mut res = 0;
    loop {
        skip_bullshit(&mut parser);
        if parser.is_eof() {
            break;
        }
        let word = parser.ascii_word();
        let mut previous_cursor = parser.pos;
        let mut go_back = false;
        if word.ends_with("mul") {
            let v = parser.advance(1);
            if v == b'(' {
                let v = parser.peek();
                if v.is_ascii_digit() {
                    let a = parser.try_parse_int();
                    let v = parser.advance(1);
                    if a.is_some() && v == b',' {
                        let v = parser.peek();
                        if v.is_ascii_digit() {
                            let b = parser.try_parse_int();
                            let v = parser.advance(1);
                            if b.is_some() && v == b')' {
                                go_back = false;
                                res += a.unwrap() * b.unwrap();
                            }
                        }
                    }
                }
            }
        }
        if go_back {
            parser.pos = previous_cursor;
        }
    }
    dbg!(res);
}

pub fn day_3_2() {
    let mut parser = Parser::new("input3.txt");

    let mut res = 0;
    let mut disable = false;
    loop {
        skip_bullshit(&mut parser);
        if parser.is_eof() {
            break;
        }
        let word = parser.ascii_word();
        let mut previous_cursor = parser.pos;
        let mut go_back = false;
        if word.ends_with("mul") {
            let v = parser.advance(1);
            if v == b'(' {
                let v = parser.peek();
                if v.is_ascii_digit() {
                    let a = parser.try_parse_int();
                    let v = parser.advance(1);
                    if a.is_some() && v == b',' {
                        let v = parser.peek();
                        if v.is_ascii_digit() {
                            let b = parser.try_parse_int();
                            let v = parser.advance(1);
                            if b.is_some() && v == b')' {
                                go_back = false;
                                if !disable {
                                    res += a.unwrap() * b.unwrap();
                                }
                            }
                        }
                    }
                }
            }
        } else if word.ends_with("do") {
            let f = parser.advance(1);
            let s = parser.advance(1);
            if f == b'(' && s == b')' {
                disable = false;
            } else {
                go_back = true;
            }
        } else if word.ends_with("don't") {
            let f = parser.advance(1);
            let s = parser.advance(1);
            if f == b'(' && s == b')' {
                disable = true;
            } else {
                go_back = true;
            }
        }
        if go_back {
            parser.pos = previous_cursor;
        }
    }
    dbg!(res);
}
