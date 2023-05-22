use proconio::input;

fn main() {
    // 入力
    input! { a: i32, b: i32 }

    let mut answer = 0;
    for i in 1..b + 1 {
        if shou_mondai(a, b, i) {
            answer = i;
        }
    }
    print!("{}", answer);
}

/**
 * 小問題 t を解く関数
 */
fn shou_mondai(a: i32, b: i32, t: i32) -> bool {
    let cl = (a + t - 1) / t; // a ÷ t の小数点以下切り上げ
    let cr = b / t; // b ÷ t の小数点以下切り捨て
    cr - cl >= 1
}
