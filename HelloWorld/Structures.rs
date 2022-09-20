struct Student {
    roll_no: i8,
    name: String,
    age: i8,
    class: i8
}

fn main(){
    let mut aviraj = Student {
        roll_no: 0,
        name: String::from("Aviraj Khare"),
        age: 18,
        class: 12
    };

    display(&aviraj);

    // Updating age of Aviraj
    aviraj.age = 26;

    println!("Age of {} is {}", aviraj.name, aviraj.age);
}

fn display(student: &Student){
    println!("Student roll no: {}", student.roll_no);
    println!("Student name: {}", student.name);
    println!("Student age: {}", student.age);
    println!("Student class: {}", student.class);
}