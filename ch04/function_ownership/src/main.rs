fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // This line will cause a compile-time error because `s` has been moved.
    // println!("s: {}", s); 

    let mut s2 = String::from("mine!");

    s2 = takes_and_give_back(s2);
    println!("s2: {}", s2);

    let x = 5;

    makes_copy(x);

    println!("x: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
