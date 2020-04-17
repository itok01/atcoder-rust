use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut a = a;

    let mut div_count = 0;

    loop {
        let mut divided = true;

        for i in 0..n {
            if a[i] % 2 == 0 {
                a[i] /= 2;
            } else {
                divided = false;
                break;
            }
        }
        if !divided {
            break;
        } else {
            div_count += 1;
        }
    }

    println!("{}", div_count);
}
