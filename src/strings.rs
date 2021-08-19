
// Primitive str = Immutable fixed-length string somewhere in memory
//String = Growable heap allocated data structure - Use when you need to modify or own string data
// pus onto string like array
// used for systems programming
pub fn run(){
    let mut hello = String::from("Hello ");


//Get Length 
println!("Length: {}", hello.len());

//push method for single chars
hello.push('W');

//pushStr will push whole strings
hello.push_str("orld!");

//Capacity in bytes
println!("Capacity: {}", hello.capacity());

//check if empty
println!("Is empty: {}", hello.is_empty());

//contains sub string
println!("Contains 'World' {}", hello.contains("World"));

//Replace
println!("Replace: {}",hello.replace("World","There"));

//Loop through said string
for word in hello.split_whitespace(){
    println!("{}",word)
}


//string with capacity
let mut s = String::with_capacity(10);
s.push('a');
s.push('b');

//Assertion testing
assert_eq!(2,s.len());// returns bool
assert_eq!(10,s.capacity());

    println!("{}",s)
}