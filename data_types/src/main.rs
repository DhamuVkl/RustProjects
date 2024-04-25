fn main() {
    println!("Hello, world!");

    let bol = true;
    let letter = 'e';
    let letter1 = "s";
    let tuple = (8, true, 's', "we");
    let mut tuple_a = (23, false, 'D', "df");
    println!("{:?}", tuple_a);
    tuple_a.0 = 24;
    println!("{:?}", tuple_a);
    let x: f64 = 10.9;

    println!("{}", bol);
    println!("{}", letter);
    println!("{}", letter1);
    println!("{:?}", tuple);
    println!("x is {}", x);
}
