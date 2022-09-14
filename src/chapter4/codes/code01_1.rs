use proconio::input;

fn main() {
    // 入力
    input! { ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64 }

    // ベクトル BA, BC, CA, CB の成分表示を求める
    let bax = ax - bx;
    let bay = ay - by;
    let bcx = cx - bx;
    let bcy = cy - by;
    let cax = ax - cx;
    let cay = ay - cy;
    let cbx = bx - cx;
    let cby = by - cy;

    // どのパターンに当てはまるか判定する
    let mut pattern = 2;
    if bax * bcx + bay * bcy < 0.0 {
        pattern = 1;
    }
    if cax * cbx + cay * cby < 0.0 {
        pattern = 3;
    }

    // 点と直線の距離を求める
    let mut answer = 0.0;
    if pattern == 1 {
        answer = (bax * bax + bay * bay).sqrt();
    }
    if pattern == 3 {
        answer = (cax * cax + cay * cay).sqrt();
    }
    if pattern == 2 {
        let s = (bax * bcy - bay * bcx).abs();
        let bc_length = (bcx * bcx + bcy * bcy).sqrt();
        answer = s / bc_length;
    }

    // 答えの出力
    println!("{}", answer);
}
