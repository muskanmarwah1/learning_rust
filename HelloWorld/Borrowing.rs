fn main(){
    let v = vec![10, 20, 30, 40];
    print_vector(&v);
    // Line below will give an error
    // println!("First element is: {}", v[0]);

    // Let's use borrowing
    println!("First element is: {:?}", &v[0]); // passing reference
}

fn print_vector(x: &Vec<i32>){
    println!("Vector is: {:?}", x);
}