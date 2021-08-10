fn main() {
    // floating points
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Operators
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    // booleans
    let t = true;
    let f: bool = false;

    // Char noted by single quotes instead of doubles
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);

    // tuples
    let tup = (500, 6.4, 1);
    // pattern matching, destruct a tuple
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{}, {}, {}", five_hundred, six_point_four, one);

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // let a = [3; 5]; = [3, 3, 3, 3, 3]

    // accessing values in the array
    let first = a[0];
    let second = a[1];
}
