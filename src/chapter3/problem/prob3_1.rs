use proconio::input;

fn main() {
    input! { n: i32, r: i32 }

    let mut fact_n = 1;
    for i in 1..=n {
        fact_n *= i;
    }

    let mut fact_r = 1;
    for i in 1..=r {
        fact_r *= i
    }

    let mut fact_nr = 1;
    for i in 1..=n - r {
        fact_nr *= i;
    }

    println!("{}", fact_n / (fact_r * fact_nr));
}
