fn test(i: &i32) {
    println!("{}", i);
}

fn main() {
    let i = 5;
    test(&i); // ①
    println!("{}", i); // ①で使った後にも使える〜
}
