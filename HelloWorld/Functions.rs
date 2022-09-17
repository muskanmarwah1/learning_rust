fn main(){
    println!("{}", add(5555, 5555));

    let mut no:i32 = 5;
    mutate_var_to_zero(&mut no);
    println!("New mutated value is: {}", no);

    //invoking display() here
    display(String::from("Aviraj Khare"));
}

fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

// Pass by reference

fn mutate_var_to_zero(param_no:&mut i32) {
    *param_no = 0; //de reference
}

// Passing string to a function
fn display(param_name: String){
    println!("My name is {}.", param_name);
}