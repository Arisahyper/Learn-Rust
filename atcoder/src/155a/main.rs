use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };

    if a == b && a != c {
        println!("Yes");
        return;
    }
    if b == c && b != a {
        println!("Yes");
        return;
    }
    if a == c && a != b {
        println!("Yes");
        return;
    }
    println!("No");
}
