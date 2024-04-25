fn main() {
    let x = 4;
    println!("x is: {}", x);
    {
        let x = 2;
        println!("x is: {} \n", x);
    }
    let x = "hello";

    const Y: u32 = 90;

    println!("y is: {}", Y);

    println!("x is: {} \n", x);
}
