fn main() {
  let arr = [1, 2, 3, 4, 5];
  let mut arr_mut = [1, 2, 3, 4, 5];
  
  println!("{} {} {} {} {}", arr[0], arr[1], arr[2], arr[3], arr[4]);
  println!("{} {} {} {} {}", arr_mut[0], arr_mut[1], arr_mut[2], arr_mut[3], arr_mut[4]);
  
  arr_mut = [6, 7, 8, 9, 10];
  println!("{} {} {} {} {}", arr_mut[0], arr_mut[1], arr_mut[2], arr_mut[3], arr_mut[4]);
}
