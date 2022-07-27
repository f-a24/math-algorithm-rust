fn main() {
    // 四則演算
    println!("{}", 869 + 120);
    println!("{}", 869 - 120);
    println!("{}", 869 * 120);
    println!("{}", 869 / 120);

    // 剰余 (mod)
    println!("{}", 8 % 5);
    println!("{}", 869 % 120);

    // 絶対値 (abs)
    println!("{}", -45_i32.abs());
    println!("{}", 15_i32.abs());

    // 累乗 (pow)
    println!("{}", 10_i32.pow(2));
    println!("{}", 3_i32.pow(4));

    // 平方根 (sqrt)
    println!("{}", 4.0_f32.sqrt());
    println!("{}", 2.0_f32.sqrt());
}
