use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n = 1000000;
    let mut m = 0;
    for _ in 0..n {
        let px: f64 = 6.0 * rng.gen::<f64>();
        let py: f64 = 9.0 * rng.gen::<f64>();

        let dist_33 = ((px - 3.0) * (px - 3.0) + (py - 3.0) * (py - 3.0)).powf(0.5);
        let dist_37 = ((px - 3.0) * (px - 3.0) + (py - 7.0) * (py - 7.0)).powf(0.5);

        if dist_33 <= 3.0 || dist_37 <= 2.0 {
            m += 1;
        }
    }
    println!("{}", m);
}
