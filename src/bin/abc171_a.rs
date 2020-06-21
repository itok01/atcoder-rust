use proconio::input;

fn main() {
    input! {
        a: char,
    }

    let a: char = a;

    let trans_a = if a.is_lowercase() { 'a' } else { 'A' };

    println!("{}", trans_a);
}
