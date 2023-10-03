fn main() {
    // creating tuples
    let tup: (i32, u32, f64, u8) = (500, 2, 8.5, 1);
    let ( x, y, z, t) = tup;

    // destructing the tuple
    println!("The value of z is: {z}");

    println!("The value of t is: {0}", tup.3)

    // The tuple without any values receives the name of: unit ()
}
