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
