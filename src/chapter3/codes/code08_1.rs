use proconio::input;

fn main() {
    input! { n: i32, x: i32, mut a: [i32; n] }
    a.sort();

    let mut answer = "No";
    let mut left = 0;
    let mut right = n - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if a[mid as usize] == x {
            answer = "Yes";
            break;
        }
        if a[mid as usize] > x {
            right = mid - 1;
        }
        if a[mid as usize] < x {
            left = mid + 1;
        }
    }

    println!("{}", answer);
}
