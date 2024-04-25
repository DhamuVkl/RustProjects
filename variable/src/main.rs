fn main() {
    let mut x = 4;
    {
        let x = 2;
        println!("x is: {} \n", x);
    }
     x += 1;

    println!("x is: {} \n", x);
}
