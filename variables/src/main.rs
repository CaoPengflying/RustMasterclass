const MAX_POINTS: i32 = 1000;

fn main() {
    // let x = 10;
    let mut x = 10;
    println!("x is {}", x);
    x = 12;
    println!("x is {}", x);

    println!("MAX_POINTS is {}", MAX_POINTS);

    let num: u8 = 254;
    println!("num is {}", num);

    let a: (i32, f64, u8) = (500, 6.4, 1);
    println!("a is {}", a.2);
}
