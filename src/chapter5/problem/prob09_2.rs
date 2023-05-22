use proconio::input;

fn main() {
    // 入力
    input! { n: usize, mut a: [isize; n], mut b: [isize; n] }

    //  配列 a, b をソートする
    a.sort();
    b.sort();

    // 答えを求める
    let mut answer = 0;
    for i in 0..n {
        answer += (a[i] - b[i]).abs();
    }

    // 出力
    println!("{}", answer);
}
