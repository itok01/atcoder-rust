use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i8,
        d: [i8; k],
    }

    let mut pay = n;
    loop {
        let mut contains = false;
        for num in d.iter() {
            if pay.to_string().contains(num.to_string().as_str()) {
                contains = true;
            }
        }
        if contains {
            pay += 1;
        } else {
            println!("{}", pay);
            return;
        }
    }
}
