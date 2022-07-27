use proconio::input;

fn main() {
    input! { n: i32 }
    println!("{}", func(n));
}

// func(N) → func(N-1) → ... → func(0) → func(-1) → func(-2) → ... と無限に呼び出されるので、プログラムが正常に動作しない
fn func(n: i32) -> i32 {
    func(n - 1) * n
}
