fn main(){
    println!("Here we will print many things");
    println!("{}",2); //println!(2) will not work
    println!("{} {}",2,3); //println!("{}",2,3) will not work
    let x = 2; //int x = 2 will not work
    println!("{}",x); //println!(x) will not work
    // print String
    let s = "the same here";// String s = "the same here" will not work
    println!("{}",s);
     
    // here more complex print
    println!("{} {} {}",x,s,2); // println!(x,s,2) will not work
    
    let app_name = "worden word learning "; // string type
    let rating=4.5; // float type
    let is_good_app =true;// bolean type
    println!("app name is :{}",app_name);
    println!("app rating :{}",rating);
    println!("is it a good app:{}",is_good_app);


}