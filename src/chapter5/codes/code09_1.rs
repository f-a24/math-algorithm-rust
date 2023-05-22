use proconio::input;

fn main() {
    // 入力
    input! { mut n: i32 }

    let mut answer = 0;

    // 10000 円以上の間、10000 円札を使い続ける
    while n >= 10000 {
        n -= 10000;
        answer += 1;
    }

    // 5000 円以上の間、5000 円札を使い続ける
    while n >= 5000 {
        n -= 5000;
        answer += 1;
    }

    // 残った金額を 1000 円札で支払う
    while n >= 1 {
        n -= 1000;
        answer += 1;
    }

    // 答えの出力
    print!("{}", answer);
}
