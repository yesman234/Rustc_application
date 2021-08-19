//Variables hold primitave values
//Variables are immutable by default
//Rust is block scoped

pub fn run(){
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}",name,age);

    age = 38;

    println!("My name is {} and I am {}",name,age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assigning Multiple variables
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}",my_name,my_age);



}