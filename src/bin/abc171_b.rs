use proconio::input;

fn main() {
    input! {
        n: isize,
        k: isize,
        p: [isize; n],
    };

    let mut p: Vec<isize> = p;
    p.sort();

    let mut total: isize = 0;

    for i in 0..k {
        total += p[i as usize];
    }

    println!("{}", total);
}
