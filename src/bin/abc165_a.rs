use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    }

    for distance in a..(b + 1) {
        if distance % k == 0 {
            println!("OK");
            return;
        }
    }

    println!("NG");
}
