// use proconio::input;

fn main() {
    // input! {
    //     s: String,
    // };

    let s = "101";
    println!("{}", s);
    println!("{}", s.chars().count()); // 文字数出力
    println!("{}", s.chars().filter(|c| *c == '1').count());
}
