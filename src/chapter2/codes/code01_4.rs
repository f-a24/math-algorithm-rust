use proconio::input;

fn main() {
    input! { mut n: i32 }
    let mut answer = String::new();
    while n >= 1 {
        if n % 2 == 0 {
            answer = String::from("0") + &answer
        }
        if n % 2 == 1 {
            answer = String::from("1") + &answer
        }
        n = n / 2;
    }
    println!("{}", answer);
}
