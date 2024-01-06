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

fn numaric_operations() {
    // division
    let quotient = 56.7 / 32.2;
    let truncated1 = -5 / 3; // Results in -1
    let truncated2 = -5.0 / 3.0; // Results in -1

    println!("quotient {quotient}");
    println!("truncated1 {truncated1}");
    println!("truncated2 {truncated2}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder {remainder}");
}

fn character() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
}

fn tuple() {
    let tup = (500, 'a', 1, 6.4);

    let (a, b, c, d) = tup;
    println!("a: {a} / b: {b} / c: {c} / d: {d}");

    let one_of_tup = tup.0;
    println!("one_of_tup(0) {one_of_tup}");
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    let b = ['a', 'c', 'a'];
    let c = ["aa", "cc", "aa"];

    let d = ['a'; 5]; // ['a', 'a', 'a', 'a', 'a']

    let d1 = d[1];
    let d2 = d[2];
}

fn plus_one(n: i32) -> i32 {
    n + 1
}

fn functions() {
    let x = plus_one(5);
    println!("The value of x is {x}")
}

fn if_expressions() {
    // If
    let number = 3;

    if number < 3 {
        println!("The number less then 3")
    } else {
        println!("The number greater then equal 3")
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    // Condition value
    let condition = true;
    let value = if condition { '3' } else { '4' };
    println!("The condition value is {value}");

    // Loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Labeled loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // While
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // For
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    if_expressions()
}
