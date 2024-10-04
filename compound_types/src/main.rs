use std::vec;

fn main() {
    let some = || println!("Hi, I am Crab's Claw!");

    let tup = (65_535,"ansh","rustify");
    println!("{}",tup.0);
    println!("{}",tup.1);
    println!("{}",tup.2);

    // destructuring
    let (x,y,z) = tup;
    println!("{}",x);
    let s1 = String::from("example");

let s2 = s1;
    
    let arr =[1,3,5,7,9,11];
    println!("{}",arr[1]);
    
    // with explicit type
    let arr2: [i32;4] = [1,2,3,4];
    println!("{}",arr2[1]);
    
    // contiguous resizable array type aka Vector

    let vtr = vec!{1,2,3};
    println!("{:?}",vtr);

    let v: Vec<i32> = (0..8).collect();
    println!("{:?}",v);

    // 
    let mut vc=Vec::new();
    vc.push("Something");
    println!("{:?}",vc);
    
    //
    let vect = Vec::<i32>::with_capacity(5);
    println!("{:?}",vect.capacity());


    

}
