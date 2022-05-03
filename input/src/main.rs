use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
        x: i64,
    };

    let takahashi = (a * (x / (a + c)) * b) + ((x % (a + c)) * b);
    let aoki = (d * (x / (d + f)) * e) + ((x % (d + f)) * e);

    match takahashi.cmp(&aoki) {
        Ordering::Less => println!("Aoki"),
        Ordering::Greater => println!("Takahashi"),
        Ordering::Equal => println!("Draw"),
    }
}

/*
    時間 = 10
    時速 = 3
    秒速 = 4

    あまり時間:             10 % (4 + 3) = 3
    予定の時間分歩いた回数:   10 / (4 + 3) = 1
    (4 * 1 * 3) + (3 * 3)
    (4 * 3) + (3 * 3)

    takahashi = (a * (x / (a + c)) * b) + ((x % (a + c)) * b)
    aoki = (d * (x / (d + f)) * e) + ((x % (d + f)) * e)

    if takahashi > aoki {
        println!("Takahashi");
    } else if takahashi < aoki {
        println!("Aoki");
    } else {
        println!("Drow");
    }


*/
