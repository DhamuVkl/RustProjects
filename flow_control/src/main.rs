fn main() {
    let cond0 = (2 as f32) <= 2.2;
    let cond1 = true && cond0; // && => and || => or ! => not also perioratize struct was  () ! && ||

    println!("{}", cond1);

    let food = "cookie";

    if food == "cookie" {
        // () not mandotary
        println!("I like it!");
    }
}
