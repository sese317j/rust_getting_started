//Vectors - Resizable Arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //Re-assign value
    numbers[1] = 20;

    //Add on to vector
    numbers.push(6);
    numbers.push(7);

    //Pop off the last value
    numbers.pop();

    println!("{:?}", numbers);

    //Get single Val
    println!("Single Value:{}",numbers[0]);

    //Get Vector legth
    println!("Vector Length: {}",numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes",mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice:{:?}",slice);

    //Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //Loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}