use proconio::input;

fn main() {
    // 入力
    input! { h: usize, w: usize, a: [[usize; w]; h] }

    // 行の総和を計算する
    let mut gyou = vec![0; h];
    for i in 0..h {
        for j in 0..w {
            gyou[i] += a[i][j];
        }
    }

    // 列の総和を計算する
    let mut retu = vec![0; w];
    for j in 0..w {
        for i in 0..h {
            retu[j] += a[i][j]
        }
    }

    // 各マスに対する答えを計算する
    let mut answer = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            answer[i][j] = gyou[i] + retu[j] - a[i][j]
        }
    }

    // 出力
    for i in 0..h {
        println!("{:?}", answer[i])
    }
}
