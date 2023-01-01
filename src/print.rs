pub fn run() {
    // ?  Print to console
    println!("Hello from the print.rs");

    // ?  Basic formatting
    println!("{} is from {}" , "George" ,"Drama");

    // ? Positional Argumnets
    println!("{0} is from {1} and  {0} likes to {2}" ,"George" , "Drama" , "code");

    // ? Named Args
    println!("{name} likes to play {activity}" , name= "George", activity = "basketball");

    //? Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}" , 10,10,10);

    //? Placholder for debug trait
    println!("{:?}" ,(12,true,"hello"));

    // ? Basic Math
    println!("10+10={}" , 10 + 10)
}