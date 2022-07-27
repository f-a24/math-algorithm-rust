use proconio::input;

fn main() {
    input! { n: i32, mut a: [i32; n] }
    a.sort();
    for i in a.iter() {
        println!("{}", i);
    }
}
