use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        r: f64,
    }

    println!("{}", r * 2.0 * PI);
}
