use proconio::input;

fn main() {
    input! { n: i32, a: [i32; n] }

    let answer = merge_sort(a);
    for i in answer {
        println!("{}", i);
    }
}

fn merge_sort(a: Vec<i32>) -> Vec<i32> {
    if a.len() == 1 {
        return a;
    }

    let m = a.len() / 2;
    let a_dash = merge_sort(a[0..m].to_vec());
    let b_dash = merge_sort(a[m..a.len()].to_vec());

    let mut c1 = 0;
    let mut c2 = 0;
    let mut c = Vec::<i32>::new();
    while c1 < a_dash.len() || c2 < b_dash.len() {
        if c1 == a_dash.len() {
            c.push(b_dash[c2]);
            c2 += 1;
        } else if c2 == b_dash.len() {
            c.push(a_dash[c1]);
            c1 += 1;
        } else {
            if a_dash[c1] <= b_dash[c2] {
                c.push(a_dash[c1]);
                c1 += 1;
            } else {
                c.push(b_dash[c2]);
                c2 += 1;
            }
        }
    }
    c
}
