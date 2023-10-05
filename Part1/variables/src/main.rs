fn main() {
let result=10.0;// i32 by default which means signed 32 bit integer

let age:u32= 20;  // u32 means unsigned 32 bit integer
let sum:i32 = 2-30; // i32 means signed 32 bit integer 
let mark:isize=10; // isize means signed integer of size 32 or 64 bit
let count:usize=30; // usize means unsigned integer of size 32 or 64 bit
let interest:f32=8.35;
let cost:f64=15000.600;
println!("result value is {}",result);
println!("sum is {} and age is {}",sum,age);
println!("mark is {} and count is {}",mark,count);
println!("interest is {} and cost is {}",interest,cost);

//these are illegal statements
/*
let age:u32= 4-20; // u32 means unsigned 32 bit integer and 4-20 is negative
let count:usize=-30; // usize means unsigned integer of size 32 or 64 bit and -30 is negative
let age:u32= 20.0 // u32 means unsigned 32 bit integer and 20.0 is float
let count:usize=30.0; // usize means unsigned integer of size 32 or 64 bit and 30.0 is float
*/
}

/*
General rules for variable names:
1. Variable names can contain letters, numbers, and underscores.
2. Variable names can only start with letters and underscores.
3. Variable names cannot start with numbers.
4. Variable names are case sensitive.
5. Variable names cannot contain spaces.
6. Variable names cannot contain special characters.
7. Variable names cannot be keywords.

Special for Rust:
let age:u32= 20;
let   => create a new variable
age   => variable name
:     => type annotation operator 
u32   => unsigned 32 bit integer type
=     => assignment operator
20    => value
;     => end of statement
*/