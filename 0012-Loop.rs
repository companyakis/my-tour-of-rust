fn main() {
    
    let mut counter = 0;
    
    loop {
        
        if counter == 13 { break }
        
        println!("counter = {}", counter);
        
        counter += 1;
    }
    
    println!("Final counter value = {}", counter);
}

// counter = 0
// counter = 1
// counter = 2
// counter = 3
// counter = 4
// counter = 5
// counter = 6
// counter = 7
// counter = 8
// counter = 9
// counter = 10
// counter = 11
// counter = 12
// Final counter value = 13
