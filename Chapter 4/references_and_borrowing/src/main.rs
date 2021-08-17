fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("Hello");

    change(&mut s);

    println!("{}", s);

    // A problem with mutable references is that if there are more than one mutable references a data race can occur which is similar to a race condition
    // Rust stops this by not allowing code with data race to compile
    // So the following code errors;
    //let mut s = String::from("hello");
    //let r1 = &mut s;
    //let r2 = &mut s;
    // However, with the use of curly brackets to create a new scope we can allow multiple mutable references, just not simultaneous ones:
    let mut s = String::from("Hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems

    let r2 = &mut s;

    // we also cant mix immutable and mutable reference such as:
    //let mut s = String::from("hello");
    //let r1 = &s; // no problem
    //let r2 = &s; // no problem
    //let r3 = &mut s; //big problem

    // If we have a mutable and immutable references, the immutable references shouldn't expect the values to change suddenly, which is why we can't have a mix of immutable and mutable references
    // We can however have multiple immutable references as the immutable references can't affect anyone else reading of the data
}

fn calculate_length(s: &String) -> usize { // s ia a reference to a String
    s.len()
} // s goes out of scope
// But because it does not have ownership of what it refers to, noting happens


fn change(some_string: &mut String) {
    // you can't change whats in the reference as references are immutable by default
    // therefore this does not work
    //s.push_str(", world");

    some_string.push_str(", world");
    
}