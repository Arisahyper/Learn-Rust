fn ex1() {
    let a = 10; // immutable object
    let b = a; // copy
    print!("{} {}", a, b); // borrow check!! - OK
}

fn ex2() {
    let a = 10; // immutable object
    let a_ref = &a; // reference
    let a_ref_copy = a_ref; // copy reference
    print!("{} {} {}", a, a_ref, a_ref_copy); // borrow check!! - OK
}

// fn ex3() {
//     let mut a = 10; // mutable object
//     let a_mut_ref = &mut a; // mutable reference
//     let a_mut_ref_move = a_mut_ref; // move mutable reference
//     print!("{}", a_mut_ref); // borrow check!! - Error!
// }

fn main() {
    ex1();
    ex2();
}
