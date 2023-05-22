use proconio::input;

fn main() {
    // 入力
    input! { n: i32, x: i32, y: i32 }

    // 4 つの整数 (a, b, c, d) の全探索
    let mut flag = false;
    for a in 1..=n {
        for b in a..=n {
            for c in b..=n {
                for d in c..=n {
                    if a + b + c + d == x && a * b * c * d == y {
                        flag = true;
                    }
                }
            }
        }
    }

    // 答えの出力
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
