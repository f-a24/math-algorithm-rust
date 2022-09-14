fn main() {
    let mut l = 1.0;
    let mut r = 2.0;
    for i in 1..=20 {
        let m = (l + r) / 2.0;
        if m * m < 2.0 {
            l = m;
        } else {
            r = m;
        }
        println!("Step {}: m = {}", i, m);
    }
}
