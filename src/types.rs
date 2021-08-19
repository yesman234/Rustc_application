/* 
primitive data Types--
Integers: u8, i8, u16, i16, u32, u64, i64, u128, i128
Floats: f32, f 64
Boolean (bool)
Charachters (char)
Tuples
Arrays 
Vectors

Rust is statically types language, which means that it must know the types of all of the variables at compile time, however, the compiler can usually infer what type to use based on the value ansd how we use it.
*/

pub fn run(){
    // default "i32"
    let x = 1;

    //float default is "f64"
    let y = 2.5;

    //Add explicit type
    let z: i64 = 4545445454545;

    //Find max size;
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);


    //Boolean
    let is_active = true;

    //Get booolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    //get emoji Unicode
    let face = '\u{1F600}';

    println!("{:?}", (x,y,z, is_active, is_greater, a1, face))


}