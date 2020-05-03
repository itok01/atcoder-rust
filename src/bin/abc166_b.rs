use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut sunukes: HashSet<usize> = (1..(n + 1)).collect();

    for _ in 0..k {
        input! {
            d: usize,
            a: [usize; d],
        }

        for sunuke in a {
            sunukes.remove(&sunuke);
        }
    }

    println!("{}", sunukes.len());
}
