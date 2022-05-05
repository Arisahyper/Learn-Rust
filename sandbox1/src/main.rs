fn _vector_test() {
    let animal_box = vec!["cat", "dog", "bird"];

    // warn
    // for i in 0..animal_box.len() {
    //     println!("{}", animal_box[i]);
    // }

    for i in animal_box {
        println!("{}", i);
    }

    let v = vec![1, 2, 3, 4, 5, 6];
    let odd_numbers = v.iter().filter(|&x| x % 2 != 0); // イテレータ
    print!("{:?}", odd_numbers);
}

fn test() {
    let a;
    {
        a = 5;
        println!("a = {}", a);
    }
    // println!(a)  // error
}

fn str_len(s: String) -> usize {
    s.len()
}

fn test2() {
    let s = String::from("hello");
    let len = str_len(s.clone());
    println!("{}", len);
    println!("{}", s);
}

fn main() {
    test();
    test2();
}
