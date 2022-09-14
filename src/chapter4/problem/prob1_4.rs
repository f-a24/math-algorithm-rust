use proconio::input;
use std::cmp::{max, min};

fn main() {
    // 入力
    input! { x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32 }

    // cross の値を計算
    let ans1 = cross(x2 - x1, y2 - y1, x3 - x1, y3 - y1);
    let ans2 = cross(x2 - x1, y2 - y1, x4 - x1, y4 - y1);
    let ans3 = cross(x4 - x3, y4 - y3, x1 - x3, y1 - y3);
    let ans4 = cross(x4 - x3, y4 - y3, x2 - x3, y2 - y3);

    // すべて一直線上に並んでいる場合（コーナーケース）
    if ans1 == 0 && ans2 == 0 && ans3 == 0 && ans4 == 0 {
        let mut a = (x1, y1); // 点 A の座標
        let mut b = (x2, y2); // 点 B の座標
        let mut c = (x3, y3); // 点 C の座標
        let mut d = (x4, y4); // 点 D の座標
        if a > b {
            let tmp = b;
            b = a;
            a = tmp;
        }

        if c > d {
            let tmp = d;
            d = c;
            c = tmp;
        }

        if max(a, c) <= min(b, d) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
    // そうでない普通の場合
    else {
        // is_ab: 線分 AB が点 C, D を分けるか？
        // is_cd: 線分 CD が点 A, B を分けるか？
        let mut is_ab = false;
        let mut is_cd = false;
        if ans1 >= 0 && ans2 <= 0 {
            is_ab = true;
        };
        if ans1 <= 0 && ans2 >= 0 {
            is_ab = true;
        };
        if ans3 >= 0 && ans4 <= 0 {
            is_cd = true;
        }
        if ans3 <= 0 && ans4 >= 0 {
            is_cd = true;
        }

        // 答えの出力
        if is_ab && is_cd {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

// ベクトル (ax, ay) と (bx, by) の内積の大きさ
fn cross(ax: i32, ay: i32, bx: i32, by: i32) -> i32 {
    ax * by - ay * bx
}
