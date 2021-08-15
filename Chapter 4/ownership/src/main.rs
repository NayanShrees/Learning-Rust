// Ignore compiler errors such as unused variable since this proj is an example

fn main() {
    // s is not valid here, it not yet declared
    let s = "Hello"; // s is valid from this point forward
                     // you can do stuff with s

    // A string literal is not always useful since the value is hardcoded into the program
    // We can't use it to do things like take a user input since they are immutable
    // This is where String comes in.
    // String is allocated on the heap and as such is available to store an amount of text that is unknown to us at compile time
    // Creating a String from a string literal using the from function like so:
    //let mut s = String::from("Hello");

    // push_str() appends a literal to a string
    //s.push_str(", world!");

    // prints "Hello, world!"
    //println!("{}", s);

    let s = String::from("Hello"); // s is valid from this point forward

    // do stuff with s

    // when you do something like this
    let s1 = String::from("Hello");
    let s2 = s1;
    // This does not copy the contents of s1 to s2
    // Instead it copies the pointer to s2 and invalidates s1
    // so you cant use the following command since s1 is invalidated
    //println!("{}, world!", s1);
    // this is known as a move

    // Rust will never automatically create "deep" copies of data
    // Therefore any automatic copying can be assumed to be inexpensive

    // If we do want a deep copy, copy the heap data of the string not just the stack data we can use "clone"
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack only data
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // Integers are known at compile time so they are stored in the stack
    // This means there's no reason we would want to prevent x from being valid after we create the variable y since making copies of the actual values are quick to make
    // In other words there's no difference between deep and shallow copying here

    let s = String::from("Hello"); // s comes into scope

    takes_ownership(s); // s's values moved into the function and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x moved into the function, but i32 is Copy, so it's okay to still use x afterward

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("Hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of {} is {}", s2, len);
} // this scope is now over, and s is no longer valid

// "drop" is used to return the allocated memory back to the heap
// drop is called automatically at the closing bracket

// s3 goes out of scope and is dropped, s2 goes out of scope but was moved so nothing happens
// s1 goes out of scope and is dropped

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and "drop" is called
  // The backing memory is freed

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing special happens

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
