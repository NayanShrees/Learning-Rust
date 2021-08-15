fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    //Expressions and statements
    //Statements do not return values
    //Expressions evaluate to a resulting value
    //Rust is an expression-based language
    //Expressions don't end in semi colons otherwise they would be statements
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is {}, y is {}", x, y);

    let x = five();
    println!("The value of x is {}", x);

    let x = plus_one(x);
    println!("The value of x is {}", x);
    
}

//In functions parameters the type must be declared 
fn another_function(x: i32, y: i32) {
    println!("The value of x is {}.", x);
    println!("The value of y is {}.", y);
}

//Return values have to have their type declared using an arrow ->
//You can return from a function early using the return keyword and specifying a value
//Most functions return the last expression implicitly
fn five() -> i32{
    5
}

fn plus_one(x: i32) -> i32{
    return x + 1;
}
