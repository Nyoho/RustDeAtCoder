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
        n: usize,
        m: usize,
        a: [i32; n],
        bc: [[i32; 2]; m],
    }

    debug!(n);
    debug!(m);
    debug!(a);
    debug!(bc);

    let mut a = a;

    let mut max = 0;

    for j in 0..m {
        a.sort();
        debug!(a);
        let mut index: usize = 0;
        let b = bc[j][0];
        let c = bc[j][1];
        for i in 0..b {
            if c > a[i as usize] {
                a[i as usize] = c;
            } else {
                break;
            }
        }
    }

    let mut sum: i64 = 0;
    for i in 0..n {
        sum += a[i] as i64;
    }
    debug!(sum);
    result.push_str(&sum.to_string());
    result.push_str("\n");
    result
}

#[test]
fn test_case1() {
    assert_eq!(
        solve(
            "3 2
5 1 4
2 3
1 5
"
            .to_string()
        ),
        "14\n"
    );
}

#[test]
fn test_case2() {
    assert_eq!(
        solve(
            "10 3
1 8 5 7 100 4 52 33 13 5
3 10
4 30
1 4
"
            .to_string()
        ),
        "338\n"
    );
}

#[test]
fn test_case3() {
    assert_eq!(
        solve(
            "3 2
100 100 100
3 99
3 99
"
            .to_string()
        ),
        "300\n"
    );
}

#[test]
fn test_case4() {
    assert_eq!(
        solve(
            "11 3
1 1 1 1 1 1 1 1 1 1 1
3 1000000000
4 1000000000
3 1000000000
"
            .to_string()
        ),
        "10000000001\n"
    );
}
