use crate::Parser;

pub fn day_11_1() {
    let mut parser = Parser::new("input11.txt");
    let mut values = Vec::with_capacity(10000);
    while !parser.is_eof() {
        let c = parser.parse_int();
        parser.advance(1);
        values.push(c);
    }
    let blinks = 25;
    let mut digits = Vec::with_capacity(100);
    for i in 0..blinks {
        for i in 0..values.len() {
            let c = values[i];
            if c == 0 {
                values[i] = 1;
            } else {
                digits.clear();
                let mut c = c;
                while c != 0 {
                    let d = c % 10;
                    digits.push(d);
                    c /= 10;
                }
                if digits.len() % 2 == 0 {
                    let m = digits.len() / 2;
                    let mut a = 0;
                    for i in (0..m).rev() {
                        a *= 10;
                        a += digits[i]
                    }
                    let mut b = 0;
                    for i in (m..digits.len()).rev() {
                        b *= 10;
                        b += digits[i]
                    }
                    values[i] = a;
                    values.push(b);
                } else {
                    values[i] *= 2024;
                }
            }
        }
    }
    // dbg!(&values);
    dbg!(values.len());
}

// pub fn day_11_2() {
//     let mut parser = Parser::new("input11.txt");
//     let mut values = Vec::with_capacity(1000000);
//     let mut values_0 = values_for_0();
//     while !parser.is_eof() {
//         let c = parser.parse_int();
//         parser.advance(1);
//         values.push(c);
//     }
//     let blinks = 25;
//     let mut res = 0;
//     for i in 0..blinks {
//         println!("blink {}/{}", i + 1, blinks);
//         for i in 0..values.len() {
//             let c = values[i];
//             if c == 0 {
//                 let remaining = blinks - i + 1;
//                 res += values_0[remaining];
//             } else {
//                 let mut c = c;
//                 let mut d = c;
//                 let mut digits = 0;
//                 while d != 0 {
//                     digits += 1;
//                     d /= 10;
//                 }
//                 if digits % 2 == 0 {
//                     let m = digits / 2;
//                     let a = c / 10_i64.pow(m);
//                     let b = c % 10_i64.pow(m);
//                     values[i] = a;
//                     values.push(b);
//                 } else {
//                     values[i] *= 2024;
//                 }
//             }
//         }
//     }
//     res += values.len();
//     // dbg!(&values);
//     dbg!(res);
// }
//
// pub fn values_for_0() -> Vec<usize> {
//     let blinks = 75;
//
//     let mut values = Vec::with_capacity(1000000);
//     values.push(0);
//     let mut res = Vec::new();
//     for i in 0..blinks {
//         println!("blink {}/{}", i + 1, blinks);
//         for i in 0..values.len() {
//             let c = values[i];
//             if c == 0 {
//                 values[i] = 1;
//             } else {
//                 let mut c = c;
//                 let mut d = c;
//                 let mut digits = 0;
//                 while d != 0 {
//                     digits += 1;
//                     d /= 10;
//                 }
//                 if digits % 2 == 0 {
//                     let m = digits / 2;
//                     let a = c / 10_i64.pow(m);
//                     let b = c % 10_i64.pow(m);
//                     values[i] = a;
//                     values.push(b);
//                 } else {
//                     values[i] *= 2024;
//                 }
//             }
//         }
//         println!("values {}", values.len());
//         res.push(values.len());
//     }
//     res
// }
//
// pub fn values_for_0() -> Vec<usize> {
//     let blinks = 25;
//
//     let mut values = Vec::with_capacity(1000000);
//     values.push(0);
//     let mut res = Vec::new();
//     for i in 0..blinks {
//         println!("blink {}/{}", i + 1, blinks);
//         for i in 0..values.len() {
//             let c = values[i];
//             if c == 0 {
//                 values[i] = 1;
//             } else {
//                 let mut c = c;
//                 let mut d = c;
//                 let mut digits = 0;
//                 while d != 0 {
//                     digits += 1;
//                     d /= 10;
//                 }
//                 if digits % 2 == 0 {
//                     let m = digits / 2;
//                     let a = c / 10_i64.pow(m);
//                     let b = c % 10_i64.pow(m);
//                     values[i] = a;
//                     values.push(b);
//                 } else {
//                     values[i] *= 2024;
//                 }
//             }
//         }
//         println!("values {}", values.len());
//         res.push(values.len());
//     }
//     res
// }
