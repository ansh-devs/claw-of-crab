use std::vec;

fn main() {
    println!("Hi, I am Crab's Claw!");

    let tup = (65_535,"ansh","rustify");
    println!("{}",tup.0);
    println!("{}",tup.1);
    println!("{}",tup.2);

    // destructuring
    let (x,y,z) = tup;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);
    
    let arr =[1,3,5,7,9,11];
    println!("{}",arr[1]);
    
    // with explicit type
    let arr2: [i32;4] = [1,2,3,4];
    println!("{}",arr2[1]);
    
   
    

}
