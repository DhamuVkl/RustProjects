fn main() {
    let mut x = 4;
    {
        let x = 2;
        print!("x is: {} \n", x);
    }
     x += 1;

    print!("x is: {} \n", x);
}
