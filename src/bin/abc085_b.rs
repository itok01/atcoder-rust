use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }

    let available_mochi: HashSet<&usize> = d.iter().collect();

    println!("{}", available_mochi.len());
}
