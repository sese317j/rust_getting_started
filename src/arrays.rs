// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    //Array of arrays (nice)
    let numbers2: [[i32;1]; 5] = [[1],[2],[3],[4],[5]];

    //Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single Val
    println!("Single Value:{}",numbers[0]);

    //Get Array legth
    println!("Array Length: {}",numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes",mem::size_of_val(&numbers));

    //size of array of arrays
    println!("Array2 occupies {} bytes",mem::size_of_val(&numbers2));


    //Get Slice
    let slice: &[i32] = &numbers[0..2];
    let slice2: &[i32] = &numbers[0..3];
    println!("Slice:{:?}", slice);
    println!("Slice2:{:?}", slice2);


    let slice: &[i32] = &numbers[0..2]; //immutable reference = copy
    println!("Slice:{:?}", slice);
    numbers[0] = 12;
    /*
    println!("Slice:{:?}", slice); -> Would not work because slice is only a copy, if after the copy is taken the original is changed
    the copy wont be altered, the borrow checker knows this and wont compile.
    */


    //Get mutable Slice, change something
    let slice: &mut [i32] = &mut numbers[0..3];
    println!("Mutable Slice before:{:?}", slice);
    slice[2]=33;
    println!("Mutable Slice after:{:?}", slice);


    //the original array also changes:
    println!("Numbers:{:?}",numbers);
}