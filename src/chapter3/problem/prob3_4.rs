use proconio::input;

fn main() {
    input! { n: i32, a: [usize; n] }

    let mut cnt = [0; 100000];
    for i in a.iter() {
        cnt[*i] += 1;
    }

    let mut answer = 0;
    for i in 1..50000 {
        answer += cnt[i] * cnt[100000 - i];
    }
    answer += cnt[50000] * (cnt[50000] - 1) / 2;

    println!("{}", answer);
}
