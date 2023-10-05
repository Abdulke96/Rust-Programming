fn main() {
    // let height:u8=257; // error: literal out of range for u8 by 1
    // let score:u8= 300; // error: literal out of range for u8 by 44
     let result=40.00;//f64 by default
    let cost:f64=8000.6980; // floating point number
    //let account_balance:f32= 8; // error: mismatched types expected f32, found integer 
    let total_sum:f32= 833_222_20.292_272; // floating point number with number separator
    let is_rust_fun:bool=true; // boolean
    let the_symbol_used_in_email:char='@'; // character
    let my_name:&str="Rust"; // string slice
    let rust_age:u8=5;
 //   println!("height: {}",height);
  //  println!("score: {}",score);
    println!("result: {}",result);
    println!("cost: {}",cost);
   // println!("account_balance: {}",account_balance);
    println!("total_sum: {}",total_sum);
    println!("is_rust_fun: {}",is_rust_fun);
    println!("the_symbol_used_in_email: {}",the_symbol_used_in_email);
    println!("my_name: {}",my_name);
    println!("rust_age: {}",rust_age);
    //rust_age = 6; // error: cannot assign twice to immutable variable `rust_age`
    //println!("rust_age: {}",rust_age);

// Still lets try more examples
const PI:f64=3.14; // rust constant
let mut number_of_students:u8= 40; // mutable variable
println!("Pi: {}",PI);
println!("number_of_students: {}",number_of_students);
number_of_students = 50; //no error as it is mutable
println!("number_of_students: {}",number_of_students);// mutable variable

}
