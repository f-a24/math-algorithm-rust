use proconio::input;
use std::f64::consts::PI;

fn main() {
    // 入力
    input! { a: f64, b: f64, h: f64, m: f64 }

    // 座標を求める
    let angle_h = 30.0 * h + 0.5 * m;
    let angle_m = 6.0 * m;
    let hx = a * (angle_h * PI / 180.0).cos();
    let hy = a * (angle_h * PI / 180.0).sin();
    let mx = b * (angle_m * PI / 180.0).cos();
    let my = b * (angle_m * PI / 180.0).sin();

    let aaa = (1.0, 2.0);
    let bbb = (3.0, 1.0);

    if aaa > bbb {
        println!("AAA");
    } else {
        println!("BBB");
    }

    // 答えを出力
    let d = ((hx - mx).powi(2) + (hy - my).powi(2)).sqrt();
    println!("{}", d);
}
