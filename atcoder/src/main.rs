use proconio::input;

#[allow(non_snake_case)]
fn main1() {
    input! {
        N: i64,
        A: [i64; N]
    }

    let ans = A.iter().map(|a| a.trailing_zeros()).min().unwrap();
    println!("{}", ans);
}

fn main2() {
    input! {
        n: u32,
        mut a: [u32; n],
    }
    let mut ans = 0;
    loop {
        if a.iter().any(|&x| x % 2 != 0) {
            break;
        }
        a = a.iter().map(|&x| x / 2).collect();
        ans += 1;
    }
    println!("{}", ans);
}

fn main() {
    let mut a = [1, 2, 3];
}
