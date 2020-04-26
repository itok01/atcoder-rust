use proconio::input;

fn main() {
    input! {
        s: usize,
        w: usize,
    }

    if s <= w {
        println!("unsafe");
    } else {
        println!("safe");
    }
}
