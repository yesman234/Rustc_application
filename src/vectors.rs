//Arrays - Fixed list where elements are the same data types
// define type and size
use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    //Re-assign values
    numbers[2] = 20;

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //pop last val
    numbers.pop();

    println!("{:?}", numbers);

    //Get single val
    println!("Single Value: {}", numbers[0]);

    //Get vector length
    println!("Vector Length: {}", numbers.len());

    //vectors are stack allocated
    println!("Vector occupies {} bytes",mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers;
    println!("Slice: {:?}",slice);

    //Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //Loop & mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);

    
}