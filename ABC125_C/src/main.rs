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

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(src: String) -> String {
    let mut result = String::from("");
    input! {
        source = src,
        n: usize,
        a: [u64; n],
    }

    debug!(n);
    debug!(a);
    let mut left_accumulated_gcds = vec![0; n];
    let mut right_accumulated_gcds = vec![0; n];
    left_accumulated_gcds[0] = a[0];
    for i in 1..n {
        left_accumulated_gcds[i] = gcd(left_accumulated_gcds[i - 1], a[i]);
    }
    right_accumulated_gcds[n - 1] = a[n - 1];
    for i in (0..n - 1).rev() {
        right_accumulated_gcds[i] = gcd(right_accumulated_gcds[i + 1], a[i]);
    }
    debug!(left_accumulated_gcds);
    debug!(right_accumulated_gcds);

    let mut max = 0;
    for i in 0..n {
        let x = if i == 0 {
            right_accumulated_gcds[1]
        } else if i == n - 1 {
            left_accumulated_gcds[n - 2]
        } else {
            gcd(left_accumulated_gcds[i - 1], right_accumulated_gcds[i + 1])
        };
        if x > max {
            max = x
        }
    }
    debug!(max);
    result.push_str(&max.to_string());
    result.push_str("\n");
    result
}

#[test]
fn test_case1() {
    assert_eq!(
        solve(
            "3
7 6 8\n"
                .to_string()
        ),
        "2\n"
    );
}

#[test]
fn test_case2() {
    assert_eq!(
        solve(
            "3
12 15 18\n"
                .to_string()
        ),
        "6\n"
    );
}

#[test]
fn test_case3() {
    assert_eq!(
        solve(
            "2
1000000000 1000000000\n"
                .to_string()
        ),
        "1000000000\n"
    );
}
