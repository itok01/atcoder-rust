use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        ab: [[usize; 2]; m],
    }

    let mut good_observatories: HashSet<usize> = (1..(n + 1)).collect();

    for _ab in ab {
        if h[_ab[0] - 1] > h[_ab[1] - 1] {
            good_observatories.remove(&_ab[1]);
        } else if h[_ab[0] - 1] < h[_ab[1] - 1] {
            good_observatories.remove(&_ab[0]);
        } else {
            good_observatories.remove(&_ab[0]);
            good_observatories.remove(&_ab[1]);
        }
    }

    println!("{}", good_observatories.len());
}
