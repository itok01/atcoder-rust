use proconio::input;

fn main() {
    input! {
        n: isize,
        m: isize,
        a: [isize; m],
    }

    let mut answer = n;

    for num in a {
        answer -= num;
    }

    if answer < 0 {
        answer = -1;
    }
    println!("{}", answer);
}
