fn main(){
    for x in 1..11 {
        println!("Number: {}", x);
    }

    let mut y = 0;

    while y <= 10 {
        println!("Increment value: {}", y);
        y += 1;
    }

    let mut z = 0;

    loop {
        println!("Value: {}", z);
        z += 1;

        if z == 10 {
            break;
        }
    }

    // Continue statement is used to pass all the below commands
    let mut count = 0;
    for num in 1..21 {
        if num % 2==0 {
            continue;
        }
        count+=1;
    }
    println! (" The count of odd values between 0 and 20 is: {} ",count);
}
