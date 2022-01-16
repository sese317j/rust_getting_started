

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

    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}",(x,y,z, is_active, is_greater,a1,face));
}