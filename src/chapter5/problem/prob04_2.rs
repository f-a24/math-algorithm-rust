use proconio::input;

fn main() {
    // 入力
    input! { n: isize, k: usize, v: [isize; k] }

    // ビット全探索
    let mut answer = 0;
    for i in 1..1 << k {
        let mut cnt = 0;
        let mut lcm = 1;
        for j in 0..k {
            if (i & (1 << j)) != 0 {
                cnt += 1;
                lcm = get_lcm(lcm, v[j])
            }
        }
        // num は n 以下の中で選ばれたすべての倍数であるものの個数
        let num = n / lcm;
        if cnt % 2 == 1 {
            answer += num;
        } else {
            answer -= num;
        }
    }

    print!("{}", answer);
}

// 最大公約数を返す関数
fn gcd(mut a: isize, mut b: isize) -> isize {
    while a >= 1 && b >= 1 {
        if a < b {
            b %= a; // a < b の場合、大きい方 b を書き換える
        } else {
            a %= b // a >= b の場合、大きい方 a を書き換える
        }
    }
    if a >= 1 {
        return a;
    }
    b
}

// 最小公倍数を返す関数
fn get_lcm(a: isize, b: isize) -> isize {
    (a / gcd(a, b)) * b
}
