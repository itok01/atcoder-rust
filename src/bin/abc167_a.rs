use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    if s == &t[0..s.len()] {
        println!("Yes");
    } else {
        println!("No");
    }
}
