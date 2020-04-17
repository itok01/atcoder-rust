use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let mut available_num: Vec<u32> = Vec::new();

    for i in 0..(n + 1) {
        let num: Vec<_> = i
            .to_string()
            .chars()
            .map(|a| a.to_digit(10).unwrap())
            .collect();

        let sum = num.iter().fold(0, |a, b| a + b);
        if a <= sum && sum <= b {
            available_num.push(i);
        }
    }

    let answer = available_num.iter().fold(0, |a, b| a + b);

    println!("{}", answer);
}
