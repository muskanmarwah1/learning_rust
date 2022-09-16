fn main(){
    let a = 11;

    if a % 2 == 0 {
        println!("Number {} is even.", a);
    } else {
        println!("Number {} is odd.", a);
    }

    // match statement
    let name = "Aviraj";

    let name_guess = match name {
        "Aviraj" => { println!("Aviraj is matched name.") },
        "John" => { println!("John is matched name.") },
        _ => { println!("Unknown name.") }
    };

    println!("Name is: {:?}", name_guess);
}