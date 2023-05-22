use proconio::input;

fn main() {
    // 入力
    input! { n: usize, s: String }

    // '(' の数 - ')' の数を depth とする
    // 途中で depth が負になったら、この時点で No
    let mut depth = 0;
    let mut flag = true;
    for i in 0..n {
        let ch = s.chars().nth(i).unwrap();
        if ch == '(' {
            depth += 1;
        }
        if ch == ')' {
            depth -= 1;
        }
        if depth < 0 {
            flag = false;
        }
    }

    // 最後、depth = 0 ['(' と ')' の数が同じ] であるかも追加で判定する
    if flag && depth == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
