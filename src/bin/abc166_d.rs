use proconio::input;

fn main() {
    input! {
        x: i128,
    }

    let mut a: i128 = 1;
    let mut b: i128 = 0;

    loop {
        if a.pow(5) < x {
            a += 1;
        } else {
            break;
        }
    }

    loop {
        if (a - b.pow(5)) > x {
            b += 1;
        } else {
            break;
        }
    }

    if check(a, b, x) {
        println!("{} {}", a, b);
        return;
    }

    let mut w = 1;

    loop {
        for w1 in 0..(w + 1) {
            for w2 in 0..(w + 1) {
                if check(a + w1, b + w2, x) {
                    println!("{} {}", a + w1, b + w2);
                    return;
                }

                if check(a + w1, b - w2, x) {
                    println!("{} {}", a + w1, b - w2);
                    return;
                }

                if check(a - w1, b - w2, x) {
                    println!("{} {}", a - w1, b - w2);
                    return;
                }
            }
        }

        w += 1;
    }
}

fn check(a: i128, b: i128, x: i128) -> bool {
    if (a.pow(5) - b.pow(5)) == x {
        return true;
    } else {
        return false;
    }
}
