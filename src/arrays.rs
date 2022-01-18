// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    //Array of arrays (nice)
    let numbers2: [[i32;2]; 5] = [[1,2],[1,2],[1,2],[1,2],[1,2]];

    //Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single Val
    println!("Single Value:{}",numbers[0]);

    //Get Array legth
    println!("Array Length: {}",numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes",mem::size_of_val(&numbers));

    //Get Sclice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice:{:?}",slice);

    //How to "unborrow ? " TODO: find out how borrowing works
    //numbers[0] = 16;
    //println!("Slice:{:?}",slice);
}