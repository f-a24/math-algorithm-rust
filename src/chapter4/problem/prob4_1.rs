fn main() {
    let n = 1000;
    let f64n = f64::from(n);
    let mut answer = 0.0;

    for i in 0..n {
        let x = 1.0 * (2.0 * f64::from(i) + 1.0) / (2.0 * f64n);
        let value = 2.0_f64.powf(x * x);
        answer += value;
    }

    println!("{}", answer / f64n);
}
