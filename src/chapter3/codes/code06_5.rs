use proconio::input;

fn main() {
    input! { n: usize, a: [i32; n] }
    println!("{}", solve(0, n, a));
}

fn solve(l: usize, r: usize, a: Vec<i32>) -> i32 {
    if r - l == 1 {
        return a[l];
    }
    let m = (l + r) / 2;
    let s1 = solve(l, m, a.clone());
    let s2 = solve(m, r, a.clone());
    s1 + s2
}
