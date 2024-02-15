fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Convention for constants just UPPERCASES
    // Constants != immutable values;
    // Constants always are type annotated
    // Constants always are equal to a constant value.
    // We can use numeric literals instead of 100000 we can use 100_000
    const SUSCRIBER_ACCOUT: u32 = 100_000;

    // Shadowing allows us to create a new variable using an existing name
    // Let's assume we want to create a string with the value of a string
    // We preserve the immutability
    // We cam change types we moved from a i32 to a &String
    let x = "six";
    println!("The value of x is: {}", x);

    // Scalar data types and compound datatypes
    // Scalar data types = a single value: Integers, Floating-point numbers, Booleans and characters
    // We can define values using:
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Bype (u8 only)

    // Integer overflow, lets assume we have a u8 value, it's maximum possible value should be 255, if 
    // it is bigger, Rust will panic in debug builds and in release build imagine that we're in a cycle
    // for 256 the value will be 0, 257 will be 1 and so on

    // Compound data types = set of values
    // Examples a tuple we declare it using the comma inside the parenthesis. 
    let tup: (&str, i32) = ("Let's Get Rusty!", 100_000);
    // Destructuring, we use parenthesis:
    let (channel, sub_count) = tup;
    println!("channel: {}, sub_count: {}", channel, sub_count);
    // We can use the operator dot "." to get values
    let channel = tup.0;
    let sub_count = tup.1;
    println!("channel: {}, sub_count: {}", channel, sub_count);
    // For arrays we use [] and every element must be separated by commas, arrays are fix size
    // If you want something that can change in time, you should use a vector
    let error_codes = [200, 404, 500];
    // standard brackets syntax to access an eleemnt
    let not_found = error_codes[1];
    println!("Result: {}", my_function(1, 2));
    conditions();
    // We can use a if-else statement in a let
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number);
    using_loop();
    using_while();
    using_for_end();
}

// functions, need the fn keyword then the name
// Rust uses the snake case convention for function names, use lowercase and separate words by underscore
// In rust every piece of code is a statement or an expression
// Statement performs an action but no return value
// Expression returns a value
fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let sum = x + y;
    // return sum;
    // in rust every last line of code without semi-colon is returned. So we don't need to do return sum;
    sum
}

fn conditions() {
    let number = 5;

    // The condition must be explicitly a boolean
    if number < 10 {
        println!("First condition was true");
    } else if number < 22 {
        println!("Second condition was true");
    } else {
        println!("Condition was false");
    }
}

// Control flow
// Loops
fn using_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("Finished! {}", result);
}

// While
fn using_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Finished! {}", number);
}

// For end loop, when you need go through a collection of items
fn using_for_end() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is {}", element);
    }

    // it goes through numbers 1, 2, 3 the right element is exclusive 
    for number in (1..4) {
        println!("{}!", number);
    }
}