fn variable() {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = 6; // shadowing
    println!("The value of x is: {}", x);
}

fn various_type() {
    let a = 12;
    let b = 12u8;
    let c = 1.2;
    let d = 1.2f32;
    let e = "Hello";
    let f = true;
    let g = (1, "hello");
    let h = [1, 2, 3];

    println!(
        "{} {} {} {} {} {} {} {} {} {}",
        a, b, c, d, e, f, g.0, h[0], a, b
    );
}

const PI: f64 = 3.14; // 定数は明示的な型指定が必要 : 外から参照可能

fn array() {
    let nums = [1, 2, 3, 4, 5];
    println!("{:?}", nums); // :? はデバッグ用
    println!("{}", nums[0]);
}

fn add_num(x: i32, y: i32) -> i32 {
    x + y // returnは省略可能
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn make_nothing() -> () {
    ()
}

fn make_nothing2() {} // 戻り値は()と推論されている

fn main() {
    variable();
    various_type();
    println!("PI is {}", PI);
    array();
    println!("{}", add_num(1, 2));

    let result = swap(1, 9);
    let (x, y) = result;
    println!("{} {}", x, y);

    let a = make_nothing();
    let b = make_nothing2();
    println!("{:?} {:?}", a, b);
}

/*
    Rustは99%型推論できる
    変数名: スネーク_ケース
*/
