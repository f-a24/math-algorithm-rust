use proconio::input;

fn main() {
    input! { n: i32, arr: [i32; n] }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    for i in arr.iter() {
        match i {
            100 => a += 1,
            200 => b += 1,
            300 => c += 1,
            400 => d += 1,
            _ => (),
        }
    }

    println!("{}", a * d + b * c);
}
