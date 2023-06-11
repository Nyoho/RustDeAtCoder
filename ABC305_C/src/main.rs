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
        s: [Chars; h],
    }

    dbg!(h, w);

    let mut x_min = w;
    let mut x_max = 0;
    let mut y_min = h;
    let mut y_max = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                if x_min > j {
                    x_min = j;
                }
                if x_max < j {
                    x_max = j;
                }
                if y_min > i {
                    y_min = i;
                }
                if y_max < i {
                    y_max = i;
                }
            }
        }
    }
    dbg!(x_min, x_max, y_min, y_max);

    for i in y_min..=y_max {
        for j in x_min..=x_max {
            if s[i][j] == '.' {
                result.push_str(&(i + 1).to_string());
                result.push_str(" ");
                result.push_str(&(j + 1).to_string());
                result.push_str("\n");
                return result;
            }
        }
    }
    result
}

#[test]
fn test_case1() {
    assert_eq!(
        solve(
            "5 6
......
..#.#.
..###.
..###.
......
"
        ),
        "2 4\n"
    );
}

#[test]
fn test_case2() {
    assert_eq!(
        solve(
            "3 2
#.
##
##
"
        ),
        "1 2\n"
    );
}

#[test]
fn test_case3() {
    assert_eq!(
        solve(
            "6 6
..####
..##.#
..####
..####
..####
......
"
        ),
        "2 5\n"
    );
}
