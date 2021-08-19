//Conditionals - Used to check the condition of something and act on the result 


pub fn run(){
    let age = 30;
    let check_id: bool = true;
    let know_person_of_age = true;

    //If /Else
    if age >= 21 && check_id || know_person_of_age {
        println!("Bartender: What would you like to drink?")
    }else if age < 21 && check_id{
        println!("Bartender: Sorry, you have to leave");
    }else{
        println!("Bartender: I'll need to see some ID")
    }
    // shorthand if
let is_of_age = if age >= 21 {true} else {false};
println!("Is Of Age: {}", is_of_age);

}