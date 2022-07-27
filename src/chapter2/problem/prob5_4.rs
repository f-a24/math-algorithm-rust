use proconio::input;

fn main() {
    input! { n: i32 }
    for i in 2..=n {
        if is_prime(i) {
            if i >= 3 {
                print!(" ");
            }
            print!("{}", i);
        }
    }
    println!();
}

fn is_prime(n: i32) -> bool {
    for i in 2..=n - 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
