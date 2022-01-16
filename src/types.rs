/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time,
// however, the compiler can usually infer what type we want to use based on the value and how we use it.


pub fn run(){
    //Default int
    let x = 2;

    //Default float
    let y = 2.5;

    //Explicit type
    let z:i64 = 4545445544;

    //Find max size
    println!("Max i32:{}", i32::MAX);
    println!("Max i64:{}", i64::MAX);

    //Boolean
    let is_active = true;

    //get boolean from expression
    let is_greater = 10>5;

    //char (also unicode)
    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}",(x,y,z, is_active, is_greater,a1,face));
}