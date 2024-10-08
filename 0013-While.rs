fn main() {

    let mut counter: i8 = 0;
    
    while counter != -12 {
        
        counter -= 1;
        
        println!("Counter: {}", counter);
    }

    println!("Counter final value: {}", counter)

}

// Counter: -1
// Counter: -2
// Counter: -3
// Counter: -4
// Counter: -5
// Counter: -6
// Counter: -7
// Counter: -8
// Counter: -9
// Counter: -10
// Counter: -11
// Counter: -12
// Counter final value: -12
