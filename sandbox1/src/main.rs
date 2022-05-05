fn _variable() {
    let a = { 1 }; // 原本 不変
    let mut b = { 2 }; // 原本 可変
    let c = &{ 3 }; // 仮 不変
    let d = &mut { 4 }; // 仮 可変

    println!("{}", a);
    println!("{}", b);
    println!("{}", *c);
    println!("{}", *d);
}

fn _borrowing() {
    let a = "hello"; // &を付けることで不変になる
    let b = a;
    println!("{} {}", a, b);

    let x = String::from("hello");
    // let y = x;   // error 所有権のムーヴはできない
    let y = &x; // ok
    println!("{} {}", x, y);
}

fn _mut_borrowing() {
    let mut x = String::from("hello");
    let y = &mut x; //
                    // let z = &mut x; // error

    // println!("{}", x); // yに渡してるから使えない
    println!("{}", y);
    // println!("{}", z);
}

fn main() {
    _variable();
    _borrowing();
    _mut_borrowing();
}

/*
    # 所有権 について

    - 原本
        -
    - 仮
        - 仮の所有権から仮の所有権を作成できる
        - 自明だが原本にはならない

    原本 -> 仮 -> 仮 -> 仮

    #
    - イミュータブル
        - 不変
    - ミュータブル
        - 可変

    # 借用
        所有権を移さずに、変数の中身を一定期間レンタルすること

        - 条件
            - イミュータブルな借用なら複数できる -> 不変な借用はいくらでもできる
            - ミュータブルな借用なら一つだけできる -> 可変な借用は一つだけできる
            - 上記二つの借用は同時にはできない
*/
