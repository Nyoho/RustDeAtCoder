#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unknown_lints)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::cmp;
use std::collections::{HashMap, VecDeque};
use std::io::{stderr, stdin, stdout, BufReader, BufWriter, Read, Write};
use std::process::exit;

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
    ($($arg:expr),*) => {
        #[cfg(debug_assertions)]
        {
            use std::io::*;
            $(writeln!(stderr(), "{} = {:?}", stringify!($arg), $arg).unwrap();)*
        }
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($($arg:expr),*) => {};
}

#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

macro_rules! stdin {
    () => {{
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }};
}

fn main() {
    print!("{}", solve(stdin!()));
}

/* ---------- */

fn solve(src: String) -> String {
    let mut result = String::from("");
    input! {
        source = src,
        s: chars,
    }

    let mut s = s;
    let n = s.len();
    let mut counter = 0;

    if n >= 3 {
        loop {
            let mut hit = false;
            for i in 0..(n - 2) {
                if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
                    s[i] = 'B';
                    s[i + 1] = 'C';
                    s[i + 2] = 'A';
                    counter += 1;
                    hit = true;
                }
            }
            if !hit {
                break;
            }
        }
    }
    debug!(counter);
    result.push_str(&counter.to_string());
    result.push_str("\n");
    result
}

#[test]
fn test_case1() {
    assert_eq!(solve("ABCABC".to_string()), "3\n");
}

#[test]
fn test_case2() {
    assert_eq!(solve("C".to_string()), "0\n");
}

#[test]
fn test_case3() {
    assert_eq!(solve("ABCACCBABCBCAABCB".to_string()), "6\n");
}
