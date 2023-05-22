use proconio::input;

fn main() {
    // 入力
    input! { n: usize, k: usize, a: [usize; n] }

    let s: usize = a.iter().sum();

    if s % 2 != k % 2 {
        println!("No");
    } else if s > k {
        println!("No");
    } else {
        println!("Yes");
    }
}
