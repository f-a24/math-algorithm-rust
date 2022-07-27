fn main() {
    let mut cnt: i32 = 1000;
    fn func1() -> i32 {
        2022
    }
    fn func2(cnt: &mut i32, pos: i32) -> i32 {
        *cnt += 1;
        *cnt + pos
    }

    println!("{}", func1());
    println!("{}", func2(&mut cnt, 500));
    println!("{}", func2(&mut cnt, 500));
}
