use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
    }

    let mut immediate_subordinates_count: Vec<usize> = Vec::with_capacity(n);

    for _ in 0..n {
        immediate_subordinates_count.push(0);
    }

    for boss_number in a.iter() {
        immediate_subordinates_count[*boss_number - 1] += 1;
    }

    for count in immediate_subordinates_count {
        println!("{}", count);
    }
}
