use proconio::input;

fn main() {
    input! {
        a: i8,
        b: i8,
        c: i8,
    }

    let num = vec![a, b, c];

    let five_count = num.iter().filter(|&n| *n == 5).count();
    let seven_count = num.iter().filter(|&n| *n == 7).count();

    if five_count == 2 && seven_count == 1 {
        println!("YES");
    } else {
        println!("NO");
    }
}
