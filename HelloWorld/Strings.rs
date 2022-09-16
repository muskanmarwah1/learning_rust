// Nice tutorial on String: https://www.tutorialspoint.com/rust/rust_string.htm

fn main(){
    let name:&str = "Hello there";
    println!("{}", name);

    let mut new_string = String::from("My name is Aviraj");
    println!("{}", new_string);
    println!("Length of String is: {}", new_string.len());

    new_string = String::from(" Khare.");
    println!("{}", new_string);

    let mut a = String::new();
    a.push_str("Avira");
    println!("{}", a);

    let n1 = "Aviraj".to_string();
    let n2 = " Khare".to_string();

    let n3 = n1 + &n2;
    println!("{}", n3);

    let a1 = "Hello".to_string();
    let a2 = "World".to_string();

    let n4 = format!("{} {}", a1, a2);
    println!("{}", n4);
}