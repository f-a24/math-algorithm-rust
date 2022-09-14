use proconio::input;

fn main() {
    // 入力
    input! { n: usize }

    let mut a = vec![0; n + 1];
    a[1] = 1;
    a[2] = 1;

    for i in 3..=n {
        a[i] = a[i - 1] + a[i - 2];
    }

    println!("{}", a[n] % 1000000007);
}
