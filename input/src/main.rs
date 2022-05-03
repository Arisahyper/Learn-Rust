use proconio::input;

fn main() {
    input! {
        v1: i64,
        v2: i64,
        s1: String,
        s2: String,
    }

    println!("{}", v1 + v2);
    println!("{s1} {s2}");
}
