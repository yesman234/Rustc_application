//Arrays - Fixed list where elements are the same data types
// define type and size
use std::mem;

pub fn run(){
    let mut numbers: [i32; 4] = [1,2,3,4];

    //Re-assign values
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single val
    println!("Single Value: {}", numbers[0]);

    //Get array length
    println!("Array Length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes",mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers;
    println!("Slice: {:?}",slice);
    
}