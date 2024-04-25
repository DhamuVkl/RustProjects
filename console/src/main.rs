use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    let msg = "Failed to Read";
    io::stdin().read_line(&mut input).expect(msg);
    println!("{}", input);
}
