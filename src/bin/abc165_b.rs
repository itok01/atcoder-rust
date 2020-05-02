use proconio::input;

fn main() {
    input! {
        x: u128,
    }

    let mut balance: u128 = 100;
    let mut year_count = 0;

    loop {
        if balance >= x {
            break;
        }
        balance += (balance / 100) as u128;
        year_count += 1;
    }

    println!("{}", year_count);
}
