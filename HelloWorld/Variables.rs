fn main(){
    let my_name = "Aviraj Khare";
    println!("My name is {}.", my_name);

    // Below is an example of integer overflow
    let num:u8 = 255;
    println!("This number is correct {}", num);
    /*
    let num2:u8 = 256;
    println!("Overflow will occur here {}", num2);
    */
    let float_with_separator = 11_000.555_001;
    println!("float value {}",float_with_separator);
   
    let int_with_separator = 50_000;
    println!("int value {}",int_with_separator);

    let is_true:bool = true;
    println!("This statement is {}", is_true);

    let special_character = '@'; //default
    let alphabet:char = 'A';
    let emoji:char = '😁';
   
    println!("special character is {}",special_character);
    println!("alphabet is {}",alphabet);
    println!("emoji is {}",emoji);

    // Mutable variables
    let mut num_a = 10;
    println!("Variable is {}, changing the value in next line.", num_a);
    num_a = 20;
    println!("Changed value is {}",num_a);

    // Shadowing example
    let mut num_b:i8 = 10;
    num_b = 20;
    // Rust will shadow first decleration which is expected
    println!("num_b is shadowed {}", num_b);
}