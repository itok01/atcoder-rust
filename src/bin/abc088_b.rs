use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut a: Vec<usize> = a;
    let mut alice_score = 0;
    let mut bob_score = 0;

    a.sort();
    a.reverse();

    for (i, num) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice_score += *num;
        } else {
            bob_score += *num;
        }
    }

    println!("{}", alice_score - bob_score);
}
