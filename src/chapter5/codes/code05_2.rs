use proconio::input;
use std::collections::VecDeque;

fn main() {
    // 入力
    input! { n: usize, in_k: usize, xy: [isize; n * 2] }
    let mut in_vec = VecDeque::from(xy.clone());
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..n {
        x.push(in_vec.pop_front().unwrap());
        y.push(in_vec.pop_front().unwrap());
    }

    // 左端 x、右端 x、下端 y、上端 y を全探索（それぞれの番号が i, j, k, l）
    let mut answer = isize::MAX;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                for l in 0..n {
                    // cl <= x <= cr, dl <= y <= dr の長方形
                    // 長方形を作るためには、cl < cr, dl < dr である必要がある
                    let cl = x[i];
                    let cr = x[j];
                    let dl = y[k];
                    let dr = y[l];
                    if cl < cr && dl < dr {
                        if check_numpoints(n, &x, &y, cl, cr, dl, dr) >= in_k {
                            let area = (cr - cl) * (dr - dl);
                            answer = area.min(answer);
                        }
                    }
                }
            }
        }
    }

    // 答えの出力
    println!("{}", answer);
}

fn check_numpoints(
    n: usize,
    x: &Vec<isize>,
    y: &Vec<isize>,
    lx: isize,
    rx: isize,
    ly: isize,
    ry: isize,
) -> usize {
    let mut cnt = 0;
    for i in 0..n {
        // 点 (x[i], y[i]) が長方形に含まれているかどうかを判定する
        if lx <= x[i] && x[i] <= rx && ly <= y[i] && y[i] <= ry {
            cnt += 1;
        }
    }
    cnt
}
