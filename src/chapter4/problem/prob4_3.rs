fn main() {
    let mut cnt = 0.0;
    let limit = 23.0;
    let mut current = 0.0;

    while current < limit {
        cnt += 1.0;
        current += 1.0 / cnt;
    }

    println!("{}", cnt);
}
