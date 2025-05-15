fn main() {
    println!("Hello, world!");

    another_function(5, 'm');

    let x = plus_one(five());
    println!("The value of x is: {x}");
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}