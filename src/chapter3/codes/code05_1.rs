use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n = 10000;
    let mut m = 0.0;
    for _ in 0..n {
        let px: f64 = rng.gen();
        let py: f64 = rng.gen();
        if px * px + py * py <= 1.0 {
            m += 1.0;
        }
    }
    println!("{}", 4.0 * m / (n as f64));
}
