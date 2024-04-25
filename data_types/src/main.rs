fn main() {
    println!("Hello, world!");

    let x: f64 = 10.9;

    let bol = true;

    let letter = 'e';

    let letter1 = "s";

    let tuple = (8, true, 's', "we");

    let mut tuple_a = (23, false, 'D', "df");   // you can't expand the tuple even in mut once it was intilaized.

    println!("{:?}", tuple_a);

    tuple_a.0 = 24;     // also you can't change the data type (int-> char) in tuple once it was intilaized.

    println!("{:?}", tuple_a);

    let mut arr = [0, 1, 2, 3, 4];

    println!("{}", arr[3]);
    println!("{:?}", arr);
    arr[3] = 8;
    println!("{:?}", arr);


    println!("{}", bol);
    println!("{}", letter);
    println!("{}", letter1);
    println!("{:?}", tuple);
    println!("x is {}", x);
}
