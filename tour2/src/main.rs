fn if_branch() {
    // この書き方だとmatch文使おうってwarning出される
    let x = 10;
    if x < 10 {
        println!("xは10より小さい");
    } else if x == 10 {
        println!("xは10です");
    } else {
        println!("xは10より大きい");
    };
}

fn loop_() {
    let mut x = 0;
    loop {
        x += 1;
        println!("{}", x);
        if x == 10 {
            break;
        }
    }
}

fn while_() {
    let mut x = 0;
    // 条件がtrueならループする / falseで終了
    while x != 10 {
        x += 1;
        println!("{}", x);
    }
}

fn for_loop() {
    // 0から9までの10回
    for x in 0..10 {
        println!("{}", x);
    }

    // 0から5までの6回
    for x in 0..=5 {
        println!("{}", x);
    }
}

fn match_1() {
    let x = 5;
    match x {
        1 => println!("xは1です"),
        2 => println!("xは2です"),
        3 => println!("xは3です"),
        4 => println!("xは4です"),
        5 => println!("xは5です"),
        _ => println!("xは1以上5以下ではありません"), // これ書かないとエラーになる
    }
}

fn match_2() {
    let x = 50;
    match x {
        1..=10 => {
            println!("1から10までの中に{}はあります", x);
        }
        11..=20 => {
            println!("11から20までの中に{}はあります", x);
        }
        matched_num @ 21..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        _ => {
            println!("xは1以上100以下ではありません");
        }
    }
}

fn variable_loop() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 10 {
            break "10なう";
        }
    };
    println!("{}", v); // 10なう が入る
}

fn ternary() -> i32 {
    let x = 42;

    // 三項式
    let v = if x < 42 { -1 } else { 1 };

    v
}

fn ex1() -> i32 {
    let v = {
        let x = 1;
        let y = 2;
        x + y
    };
    v + 10 // return
}

fn main() {
    if_branch();
    loop_();
    while_();
    for_loop();
    match_1();
    match_2();
    variable_loop();
    println!("{}", ternary());
    println!("############################");
    println!("{}", ex1());
}
