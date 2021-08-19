//Structs -- Used to create custom data types

//Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

//Tuple Struct
//struct Color(u8,u8,u8);
struct Person{
    first_name: String,
    last_name: String
}
impl Person {
    //Construct a person object
    fn new(first: &str, last: &str)-> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //Get full name 
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name,self.last_name)
    }
    //set last name 
    fn self_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    //Name of tuple
    fn to_tuple(self) ->(String,String){
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    let mut p = Person::new("John", "Doe");
    println!("Person {}", p.full_name());

    p.self_last_name("williams");
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
   
// let mut c = Color(255, 0, 0);
// println!("Color: {} {} {}", c.0, c.1,c.2);
}