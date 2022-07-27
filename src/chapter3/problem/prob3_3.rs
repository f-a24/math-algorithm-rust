use proconio::input;

fn main() {
    input! { n: i32, a: [i32; n] }

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    for i in a.iter() {
        match i {
            1 => x += 1,
            2 => y += 1,
            3 => z += 1,
            _ => (),
        }
    }

    println!("{}", x * (x - 1) / 2 + y * (y - 1) / 2 + z * (z - 1) / 2);
}
