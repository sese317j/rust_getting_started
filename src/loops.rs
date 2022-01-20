// Loops - Used to iterate until a condition is met

pub fn run(){
    let mut count = 0;


    /*
    //Infinite loop
    loop{
        count += 1;
        println!("Number: {}",count);

        if count == 1337{
            break;
        }
    }
    */

    /*
    while x <= 100 {
        if x %15 == 0 {
            println!("fizzbuzz");
        }
        else if x % 5 == 0{
            println!("buzz");
        }
        else if x % 3 == 0{
            println!("fizz");
        }
        else {
            println!("{}", x);
        }
        //Inc
        x += 1;

    }
    */

    //For Range
    for x in 0..100{
        if x %15 == 0 {
            println!("fizzbuzz");
        }
        else if x % 5 == 0{
            println!("buzz");
        }
        else if x % 3 == 0{
            println!("fizz");
        }
        else {
            println!("{}", x);
        }
    }

}