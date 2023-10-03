fn main() {
    let mut x = 5;
    const ONE_DAY: u32 = 60 * 60 * 24;

    println!("The value of the constant: {ONE_DAY}");
    println!("The value of x is: {x}");
    x = 6;
    

    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess: {guess}");

    // Scalar Types: represent a single value.
    // Integers, floating point numbers, booleans anc characters.
    // f64 is the default for floating points
    // u32 is the default for unsigned integers

    let _x = 2.0; // f64 double-precision float
    let _y: f32 = 3.0; // f32 single-precision float

    // Explicit type annotation
    let _flag:bool = false;

    // Rust's char type is four bytes in size and represents 
    // a Unicode scalar value.
    let _z: char = 'a';

}


// Note: Rust is a statically typed language, which means
// that it must know the types of all variables at compile time. 
