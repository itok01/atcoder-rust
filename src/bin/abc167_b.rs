use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        k: i64,
    }

    let mut answer: i64 = 0;
    let mut count: i64 = k;

    if count <= 0 {
        println!("{:?}", answer);
        return;
    }

    if a < count {
        count -= a;
        answer += a;
    } else {
        answer += count;
        println!("{:?}", answer);
        return;
    }

    if count <= 0 {
        println!("{:?}", answer);
        return;
    }

    if b < count {
        count -= b;
    } else {
        println!("{:?}", answer);
        return;
    }

    if count <= 0 {
        println!("{:?}", answer);
        return;
    }

    if c < count {
        answer -= c;
    } else {
        answer -= count;
        println!("{:?}", answer);
        return;
    }

    println!("{:?}", answer);
}
