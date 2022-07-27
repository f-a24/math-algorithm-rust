use proconio::input;

fn main() {
    input! { n: i32 }
    let mut cnt = 1;
    for i in 1..=n {
        cnt *= i;
    }
    println!("{}", cnt);
}
