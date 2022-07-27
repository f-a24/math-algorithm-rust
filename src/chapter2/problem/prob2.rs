use proconio::input;

fn main() {
    input! { n: i32, a: [i32; n] }
    println!("{}", a.iter().sum::<i32>() % 100);
}
