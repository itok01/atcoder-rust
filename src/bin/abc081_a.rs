use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let marble_count = s.split("").filter(|&n| n == "1").count();

    println!("{}", marble_count);
}
