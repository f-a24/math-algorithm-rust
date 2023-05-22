use proconio::input;

fn main() {
    // 入力
    input! { mut n: i32 }
    let mut answer = 0;

    // 10000 円札を支払う
    answer += n / 10000;
    n %= 10000;

    // 5000 円札を支払う
    answer += n / 5000;
    n %= 5000;

    // 1000 円札を支払う
    answer += n / 1000;

    // 出力
    println!("{}", answer);
}
