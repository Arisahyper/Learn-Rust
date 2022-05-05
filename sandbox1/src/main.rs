fn main() {
    let animal_box = vec!["cat", "dog", "bird"];

    for i in 0..animal_box.len() {
        println!("{}", animal_box[i]);
    }

    for i in animal_box {
        println!("{}", i);
    }
}
