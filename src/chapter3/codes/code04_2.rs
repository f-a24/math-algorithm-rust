use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! { n: i32, mut a: [f64; n], mut b: [f64; n] }
    a.append(&mut b);
    let mut vec: VecDeque<f64> = VecDeque::from(a.clone());

    let mut answer = 0.0;
    for _ in 0..n {
        let p = vec.pop_front().unwrap();
        let q = vec.pop_front().unwrap();
        answer += q / p;
    }

    println!("{}", answer);
}
