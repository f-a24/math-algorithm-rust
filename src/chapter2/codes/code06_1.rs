use proconio::input;

fn main() {
    input! { n: i32, s: i32, a: [i32; n] }
    let mut answer = "No";
    for i in 0..(1 << n) {
        let mut sum = 0;
        for (j, val) in a.iter().enumerate() {
            if (i & (1 << j)) != 0 {
                sum += val;
            }
        }
        if sum == s {
            answer = "Yes";
            break;
        }
    }
    println!("{}", answer);
}
