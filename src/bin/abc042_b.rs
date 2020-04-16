use proconio::input;

fn main() {
    input! {
        n: i8,
        l: i8,
        s: [String; n],
    }

    let mut s: Vec<String> = s;

    s.sort();
    let joined = s.join("");

    println!("{}", joined);
}
