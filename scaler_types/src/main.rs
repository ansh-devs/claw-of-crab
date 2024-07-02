fn main() {
    println!("Hi, I am Crab's Claw!");

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;
    let byte = b'G';

    println!("{}",decimal);
    println!("{}",hex);
    println!("{}",octal);
    println!("{}",binary);
    println!("{}",byte);
    
    let defflt=56.5; // by defaults infers to f64 because it gives more precision
    println!("{}",defflt);
    
}
