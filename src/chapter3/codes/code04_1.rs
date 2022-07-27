use proconio::input;

fn main() {
    input! { n: usize, b: [f64; n], r: [f64; n] }
    let mut blue = 0.0;
    let mut red = 0.0;
    for i in 0..n {
        blue += b[i] / (n as f64);
        red += r[i] / (n as f64);
    }
    println!("{}", blue + red);
}
