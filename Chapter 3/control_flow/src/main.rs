fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {}", number);

    // let number = if condition { 5 } else { "six" };
    // error since expression types must match
    // As the compiler needs to know what type the variable needs to be so that it can verify throughout the code that the type is valid everywhere we use the variable
    // Makes it more safe since the let isn't bound in runtime, if this was the case the compiler can make less guarantees about the code

    let mut counter = 0;

    // "loop" loops forever
    // useful for things such as checking whether a thread has completed its job
    let result = loop{
        counter += 1;

        if counter == 10{
            //can use break to return a value
            //break to break out of a continuous loop
            break counter * 2;
        }
    };

    println!("The value of counter is {}", result);


    // conditional loops with while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is {}", a[index]);

        index += 1;
    }

    // iterates through the array
    for element in a.iter() {
        println!("The value is {}", element);
    }

    // (x..n) generates a range of x to n
    for number in (1..4).rev(){
        println!("{}!", number);
    }
}
