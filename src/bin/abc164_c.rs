use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let item: HashSet<&String> = s.iter().collect();

    println!("{}", item.len());
}
