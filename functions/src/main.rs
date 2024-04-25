fn main() {
    println!("Hello, world!");
    name();
    add(50, 82);
    let result = add_num(500, 200);
    println!("{}", result);

    let ans = add_1(50, 99);
    println!("{}", ans);

}

fn name() {
    println!("Dhamu");
}

fn add(x: u32, y: u32) {
    println!("sum is:{}", x + y);
}

fn add_num(x: u64, y: u64) -> u64 {
    // function returns value need to specifi return data type.
    x + y   // to return value avoid ; (or) use->   return x + y;
}

fn add_1(x: i64, y: i64) -> i64 {
    let result = x + y;
    if result > 10 {
        return result - 100;
    }
    return result;

}