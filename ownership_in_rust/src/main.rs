/* 
    This is one of the most amazing features in rust
    it allow us to make rust memory-safety without using the garbage collector

    Ownership rules
    1. Each value in Rust has a variable that's called its owner
    2. There can only be one owner at a time
    3. When the owner goes out of scope, the value will be dropped
*/

fn main() {
    // Creating a new scope
    { // s is not valid here, it's not yet declared
        // let s = "Hello"; // s is valid from this point forward this is a string literal and it will be stored in the stack because it has a fixed size
        let s = String::from("Hello"); // This is dinamic so it is stored in the heap
        // do stuff with s
    } // this scope is now over, and s is no loger valid

    /*
        This is now allowed due to rust ownership rules. 
        We're borrowing s to ownership() and we need to return the ownership again
        let s = String::from("hello");
        ownership(s);
        println!("{}", s);
    */

    let s = String::from("Hello");
    // Using shadowing
    let s = return_ownership(s);
    println!("{}", s);

    // The previous case it's a little bit weird we can use references to avoid return the ownership
    let s = String::from("Hello");
    using_references(&s);
    println!("{}", s);

    // We can pass mutable reference values too
    let mut s = String::from("Hello");
    mutable_references(&mut s);
    println!("{}", s);
    /*
        Mutable references have one restriction
        You can have only one particular reference to a particular piece of data in a particular scope
        let mut s = String::from("Hello");

        let r1 = &mut s;
        let r2 = &mut s; //<- error because we're using two references to the same data

        println!("{}, {}", r1, r2);
    */

    // to fix that we can return to just immutable references
    let s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    /* 
        If you want to mix mutable an immutable 
        let mut s = String::from("Hello");

        let r1 = &s;
        let r2 = &s;
        let r3 = &mut s; // <- error because until this scope we're trying to modify s

        println!("{}, {}", r1, r2);
    */

    // To solve the issue we use a mutable value after the scope of the mutable ones
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2); // End of the scope now we can declare a mutable reference

    let r3 = &mut s;
    r3.push_str(" world!");
    println!("{}", r3);

    /* 
        Rules for references
        1. At any given time, you can have either one mutable reference or any number of immutable references
        2. References must always be valid
    */

    // String Slices, but we can use slices in other things like, uint, float, etc. 
    slices_unsafe();
    slices();
}

fn memory_allocation() {
    let x = 5;
    let y = x; // This is a copy

    let s1 = String::from("Hello");
    //let s2 = s1; // Move (not shallow copy)
    let s2 = s1.clone();

    println!("{}, world!", s1);
}

fn ownership(s: String) {
    println!("ownership: {}", s);
}

fn return_ownership(s: String) -> String {
    println!("return_ownership: {}", s);
    s
}

fn using_references(s: &String) {
    println!("using_references: {}", s)
}

fn mutable_references(s: &mut String) {
    println!("mutable_references: {}", s);
    s.push_str(" world");
}

fn slices_unsafe() {
    let mut s = String::from("Hello World");
    let word = first_word_unsafe(&s);
    s.clear();
    println!("slices_unsafe: {}", word);
    // If we try to do something here with word and the index that we got, we will have issues.
}

fn first_word_unsafe(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn slices() {
    let mut s = String::from("Hello world");
    let word = first_word(&s);
    // s.clear(); // throws error because we're cleaning s and we pretend to use it later. This is mutable.
    println!("slices: {}", word);
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // or &[0..i]
        }
    }
    return &s[..];
}