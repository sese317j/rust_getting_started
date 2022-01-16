pub fn run(){
    //print to console
    println!("hello from the  print file!");

    //basic formatting
    println!("{}, zwei, drei, {}","eins","vier");

    //positional arguments
    println!("{0} ist eine Zahl, {1} ist eine zahl, und {0} ist die gleiche zahl wie die erste", "drei" , "zwei");

    //named Arguments
    println!("{zahl1} ist eine Zahl, {zahl2} ist eine zahl, und {zahl1} ist die gleiche zahl wie die erste",
             zahl1="drei" , zahl2="zwei");

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

    //Placeholder for debug trait
    println!("{:?}",(12,true,"hello"));

    //Basic math
    println!("10 + 10 = {}",10+10);
}