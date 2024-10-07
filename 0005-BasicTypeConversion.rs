fn main() {

    let a: u8 = 101;
    
    let b = 102u16;
    
    let c = true;
    
    //let sum = a + b; // error[E0277]: cannot add `u16` to `u8`
    
    let sum = a + b as u8;
    
    println!("a + b = {sum}"); // a + b = 203
    
    let sum2 = a as u16 + b + c as u16;
    
    println!("a + b + c = {sum2} ") // a + b + c = 204 
    
}
