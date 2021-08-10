// Constants are defined using the const keyword
// They can be defined in any scope even the global scope
// The data type must be annotated
// They can only be set to constant expressions (not the result of the function call), must be known at compile time
// Naming convention is all caps with underscores between words
// Underscores can be added to numeric literals to improve readability
const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The constant value MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Shadowing to change type
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The number of spaces is {}", spaces);
    // Better than using mut as there will be a compile-time error
    // let mut spaces = "    ";
    // spaces = spaces.len();
    // This is because spaces is still a string data type and not a usize data type which is what the len function returns
    // We cant mutate a variables data type
}
