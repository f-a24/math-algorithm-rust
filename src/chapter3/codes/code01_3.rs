use proconio::input;

fn main() {
    input! { n: i32 }
    let limit = (n as f32).powf(0.5) as i32;
    for i in 2..=limit + 1 {
        if n % i == 0 {
            println!("{}", i);
            if i != n / i {
                println!("{}", n / i);
            }
        }
    }
}
