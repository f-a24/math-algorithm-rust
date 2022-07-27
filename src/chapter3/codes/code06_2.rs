use proconio::input;

fn main() {
    input! { n: usize, mut a: [i32; n] }

    for i in 0..n {
        let mut min_position = i;
        let mut min_value = a[i];
        for j in i..n {
            if a[j] < min_value {
                min_position = j;
                min_value = a[j];
            }
        }
        let temp = a[i];
        a[i] = a[min_position];
        a[min_position] = temp;
    }

    for i in a.iter() {
        println!("{}", i);
    }
}
