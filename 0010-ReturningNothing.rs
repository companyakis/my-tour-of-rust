fn main() {

    let result = no_return_now();
    
    println!("Result is {:?}.", result);
    
    println!("Result is {:?}.", no_return_now())

}

fn no_return_now() -> () {
    
}

// Result is ().
// Result is ().
