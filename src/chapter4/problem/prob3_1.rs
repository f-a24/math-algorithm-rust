fn main() {
    let r = 2.0; // √2 を求めたいから
    let mut a = 2.0; // 初期値を適当に 2.0 にセットする

    for i in 1..=5 {
        // 点 (a, f(a)) の x 座標と y 座標を求める
        let zahyou_x = a;
        let zahyou_y = a * a * a;

        // 接線の式 y = sessen_a * x + sessen_b を求める
        let sessen_a = 3.0 * zahyou_x * zahyou_x;
        let sessen_b = zahyou_y - sessen_a * zahyou_x;

        // 次の a の値 next_a を求める
        let next_a = (r - sessen_b) / sessen_a;
        println!("Step {}: a = {} -> {}", i, a, next_a);
        a = next_a;
    }
}
