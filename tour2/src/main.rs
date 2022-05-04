fn if_branch() {
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

fn main() {
    if_branch();
    println!("############################");
    loop_();
    println!("############################");
    while_();
    println!("############################");
    for_loop();
    println!("############################");
}
