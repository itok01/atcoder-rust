use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp = vec![std::isize::MAX; n];
    dp[0] = 0;

    for (i, _) in h.iter().enumerate() {
        if i != (h.len() - 2) {
            let cost = (h[i] - h[i + 1]).abs() + dp[i];
            if dp[i + 1] > cost {
                dp[i + 1] = cost;
            }

            let cost = (h[i] - h[i + 2]).abs() + dp[i];
            if dp[i + 2] > cost {
                dp[i + 2] = cost;
            }
        } else {
            let cost = (h[i] - h[i + 1]).abs() + dp[i];
            if dp[i + 1] > cost {
                dp[i + 1] = cost;
            }
            break;
        }
    }

    println!("{:?}", dp.last().unwrap());
}
