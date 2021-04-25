use proconio::input;
use proconio::marker::Chars;
use proconio::source::once::OnceSource;

fn main() {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    print!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    let mut result = String::from("");

    let source = OnceSource::from(src);
    input! {
        from source,
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        s: [Chars; h],
    }

    dbg!("h:{} w:{} x:{} y:{}\n", h, w, x, y);
    let mut counter = 1;

    dbg!("(x,y)={}", s[x - 1][y - 1]);
    for i in 0..h {
        for j in 0..w {
            dbg!("{}", s[i][j]);
        }
        dbg!("\n");
    }

    // 縦検索
    for i in (0..(x - 1)).rev() {
        dbg!("{}{}", i, s[i][y - 1]);
        if s[i][y - 1] == '#' {
            break;
        }
        counter += 1;
    }
    dbg!("");
    for i in x..h {
        dbg!("{}{}", i, s[i][y - 1]);
        if s[i][y - 1] == '#' {
            break;
        }
        counter += 1;
    }
    dbg!("");
    // 横検索
    for j in (0..(y - 1)).rev() {
        dbg!("{}{}", j, s[x - 1][j]);
        if s[x - 1][j] == '#' {
            break;
        }
        counter += 1;
    }
    dbg!("");
    for j in y..w {
        dbg!("{}{}", j, s[x - 1][j]);
        if s[x - 1][j] == '#' {
            break;
        }
        counter += 1;
    }
    result.push_str(&counter.to_string());
    result.push_str("\n");
    result
}

#[test]
fn test_case1() {
    assert_eq!(
        solve(
            "4 4 2 2
##..
...#
#.#.
.#.#
"
        ),
        "4\n"
    );
}

#[test]
fn test_case2() {
    assert_eq!(
        solve(
            "3 5 1 4
#....
#####
....#
"
        ),
        "4\n"
    );
}

#[test]
fn test_case3() {
    assert_eq!(
        solve(
            "5 5 4 2
.#..#
#.###
##...
#..#.
#.###
"
        ),
        "3\n"
    );
}
