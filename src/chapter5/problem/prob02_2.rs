use proconio::input;

fn main() {
    // 入力
    input! { n: usize, k: isize, a: [usize; n] }

    // 配列の初期化
    let mut first: Vec<isize> = vec![-1; n + 1];
    let mut second: Vec<isize> = vec![-1; n + 1];

    // 答えを求める（cur は現在いる町の番号）
    let mut cnt = 0;
    let mut cur: usize = 1;
    loop {
        // first, second の更新
        if first[cur] == -1 {
            first[cur] = cnt;
        } else if second[cur] == -1 {
            second[cur] = cnt
        }

        // K 回の移動後に町 cur にいるか判定
        if cnt == k {
            println!("{}", cur);
            break;
        } else if second[cur] != -1 && (k - first[cur]) % (second[cur] - first[cur]) == 0 {
            println!("{}", cur);
            break;
        }

        // 移動
        cur = a[cur - 1];
        cnt += 1;
    }
}
