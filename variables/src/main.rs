fn mutable() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The valud of x is: {x}");
}

fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("the value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Can creating a new variable
    // let y = "Y_VALUE";
    // let y = y.len();
    // println!("Y is: {y}");

    // Can not redefine
    // let mut z = "Z_VALUE";
    // z = z.len();
    // println!("Z is: {z}");
}

fn data_types() {
    // Scalar Types: integers, floating-point numbers, Booleans, characters
    // let guess: u32 = "42".parse().expect("Not a number!");

    // https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#floating-point-types
}

fn main() {
    mutable();
    constants();
    shadowing();
    data_types();
}
