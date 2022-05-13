// use proconio::input;

fn main() {
    let n = 100;
    let mut v: Vec<f64> = Vec::with_capacity(n);
    for i in 0..n {
        v.push(std::f64::consts::PI);
    }
    // println!("{} {}", v[0], v.len());
    println!("{:?}", v);

    // // input! {
    // //     n: usize,
    // //     l: [usize; n],
    // // };
    // let n = 5;
    // let mut test: Vec<usize>;
    // for i in 0..n {
    //     input! {
    //         n: usize,
    //         l: [usize; n],
    //     };
    //     println!("{}", i);
    //     test = l;
    // }
    // println!("{:?}", test);
}
