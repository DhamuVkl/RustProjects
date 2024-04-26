use std::io;

fn main() {
    println!("Hello, world!");
    let mut x = String::new();
    let mut y = String::new();

    io::stdin().read_line(&mut x).expect("Failed");
    io::stdin().read_line(&mut y).expect("Failed");

    let check_x: i128 = x.trim().parse().unwrap();
    let check_y: i128 = y.trim().parse().unwrap();

    let z = check_x + check_y;

    println!("Sum is: {}", z);
}
