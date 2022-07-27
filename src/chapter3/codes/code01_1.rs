use proconio::input;

fn main() {
    input! { n: i32 }
    let answer = is_prime(n);
    if answer == true {
        println!("prime");
    } else {
        println!("not prime");
    }
}

fn is_prime(n: i32) -> bool {
    for i in 2..=n - 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
