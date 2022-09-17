fn main(){
    let tuple:(String, u8) = ("Aviraj Khare".to_string(), 26);
    println!("{:?}", tuple);

    // accessing tuple
    println!("Name: {:?}", tuple.0);
    println!("Age: {:?}", tuple.1);

    // invoking a function in which tuple is called
    print(tuple);

    // destructuring of a tuple
    // let (name, age) = tuple;

    // println!("Name is {} and age is {}.", name, age);
}

fn print(x:(String, u8)){
    println!("Name: {:?} and age: {:?}", x.0, x.1);
}