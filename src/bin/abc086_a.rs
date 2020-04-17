use proconio::input;

fn main() {
    input! {
        a: u16,
        b: u16,
    }

    if (a * b) % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
