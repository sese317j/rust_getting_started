//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block scoped language (Variable created in function only exists inside of the function)

pub fn run() {
    let name = "Brad";
    let mut age = 37;

    println!("My name is {} and i am {}", name,age);
    age = 38;

    println!("My name is {} and i am {}", name,age);

    //Define constance
    const ID: i32 = 001;
    println!("ID: {}",ID);

    //Assigning multiple vars
    let ( my_name, my_age ) = ("brad" , 37);
    println!("{} is {}", my_name , my_age);
}