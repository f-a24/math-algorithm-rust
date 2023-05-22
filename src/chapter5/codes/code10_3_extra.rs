use proconio::input;

fn main() {
    // 入力
    input! { n: i32, x: i32, y: i32 }

    if y > n.pow(4) {
        println!("No");
    } else {
        let mut d = Vec::new();
        for i in 1..=n {
            if y % i == 0 {
                d.push(i);
            }
        }
        let divcnt = d.len();
        let mut flag = false;
        for i in 0..divcnt {
            for j in i..divcnt {
                for k in j..divcnt {
                    for l in k..divcnt {
                        if d[i] + d[j] + d[k] + d[l] == x && d[i] * d[j] * d[k] * d[l] == y {
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
}
