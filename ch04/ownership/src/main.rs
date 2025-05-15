fn main() {
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    let x = 5;
    let mut y = x;
    {
        let x = 10;
        y = x;
        println!("x: {x}, y: {y}");
    }

    println!("x: {x}, y: {y}");
}