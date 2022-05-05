fn _variable() {
    let a = "Hello".to_string();
    let a2 = "Hello";
    let b = a;
    println!("a = {}, b = {}", a, b);
    println!("a2 = {}, b = {}", a2, b);
}

fn main() {
    _variable();
}
