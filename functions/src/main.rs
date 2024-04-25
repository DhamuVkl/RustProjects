fn main() {
    println!("Hello, world!");
    name();
    add(50, 82);
    let result = add_num(500, 200);
    println!("{}", result);
}

fn name() {
    println!("Dhamu");
}

fn add(x: u32, y: u32) {
    println!("sum is:{}", x + y);
}

fn add_num(x: u64, y: u64) -> u64 {
    // function returns value need to specifi return data type.
    x + y // to return value avoid ;
}
