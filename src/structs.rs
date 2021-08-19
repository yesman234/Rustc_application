//Structs -- Used to create custom data types

//Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

//Tuple Struct
struct Color(u8,u8,u8);

pub fn run(){
   
let mut c = Color(255, 0, 0);
println!("Color: {} {} {}", c.0, c.1,c.2);
}