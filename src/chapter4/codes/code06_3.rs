use proconio::input;

fn main() {
    // 入力
    input! { a: usize, b: usize }

    let mut answer = 1; // a の 0 乗は 1 なので、answer = 1 に初期化しておく
    for _ in 0..b {
        answer = (answer * a) % 1000000007;
    }

    println!("{}", answer);
}
