fn main(){
    let arr = [10, 20, 30, 40];

    println!("{:?}", arr);
    println!("Size of array is: {}", arr.len());

    for i in 0..arr.len() {
        println!("Array element {}: {}", i, arr[i]);
    }

    // mutable array
    let mut arr_2 = [1, 2, 3 ,4];

    println!("Array is: {:?}", arr_2);

    arr_2[1] = 5;
    println!("Mutated array is: {:?}", arr_2);

    // Pass by reference
    let mut arr_3 = [5];
    increment_array(&mut arr_3);

}

fn increment_array(arr:&mut [i32;1]){
    for i in 0..1 {
        arr[i] += 1;
    }
    println!("{:?}", arr);
}