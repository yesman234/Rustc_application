
pub fn run(){
    //print to console
    println!("This is build");

    //basic formatting
    println!("Number {}",1);

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}","Brad","Mass","code");

    //named arguments
    println!("{name} likes to play {activity}", name = "john", activity = "baseball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

    //Placeholder for debug trait
    println!("{:?}",(12 ,true, "hello"));

    //Basic Arithmetic

println!("10 + 10 = {}",10+10);
}