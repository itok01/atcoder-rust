use proconio::input;

const A_VALUE: usize = 500;
const B_VALUE: usize = 100;
const C_VALUE: usize = 50;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut count = 0;

    for i in 0..(a + 1) {
        for j in 0..(b + 1) {
            for k in 0..(c + 1) {
                if (A_VALUE * i + B_VALUE * j + C_VALUE * k) == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
