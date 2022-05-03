use std::collections::HashMap;
// import std.collections.HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert("Rust", "Is Awesome!");  // {"Rust": "Is Awesome!"}
  println!("{}", map["Rust"]);        // Is Awesome!
  map.insert("Python", "Is bad");     // {"Rust": "Is Awesome!", "Python": "Is Awesome!"}
  println!("{:?}", map);              // {"Rust": "Is Awesome!", "Python": "Is Awesome!"}
}
