use proconio::input;

fn main() {
    input! { mut n: i32 }
    let limit = (n as f32).powf(0.5) as i32;
    let mut vec: Vec<i32> = Vec::new();

    // let mut flag = false;
    for i in 2..=limit + 1 {
        while n % i == 0 {
            n /= i;
            vec.push(i);

            // if flag {
            //     print!(" ");
            // }
            // flag = true;
            // print!("{}", i);
        }
    }
    if n >= 2 {
        vec.push(n);
        // if flag {
        //     print!(" ");
        // }
        // println!("{}", n);
    }
    let answer: Vec<String> = vec.iter().map(|x| x.to_string()).collect();
    println!("{}", answer.join(" "));
}
