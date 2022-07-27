use proconio::input;

fn main() {
    input! { n: i32 }
    println!("{}", func(n));
}

fn func(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    func(n - 1) * n
}
