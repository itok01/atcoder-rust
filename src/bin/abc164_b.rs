use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }

    let mut takahashi_monster_health = a;
    let mut aoki_monster_health = c;

    loop {
        aoki_monster_health -= b;
        if aoki_monster_health <= 0 {
            println!("Yes");
            break;
        }

        takahashi_monster_health -= d;
        if takahashi_monster_health <= 0 {
            println!("No");
            break;
        }
    }
}
