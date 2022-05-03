fn main() {
    let v1 = 50;
    let v2 = 10;

    let s1 = "Hello";
    let s2 = "World";

    // 算術演算
    println!("+ {}", v1 + v2);
    println!("- {}", v1 - v2);
    println!("* {}", v1 * v2);
    println!("/ {}", v1 / v2);
    println!("% {}", v1 % v2);

    println!("<< {}", v1 << 2);

    // 論理演算
    println!("{}", v1 & v2);
    println!("{}", v1 | v2);
    println!("{}", v1 ^ v2);

    println!("{}", s1 == s2);
}

/*
AND
110010 = 50
001010 = 10
000010 = 2
000010 = 2

OR
110010 = 50
001010 = 10
111010 = 58

*/
