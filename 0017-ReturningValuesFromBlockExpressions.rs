fn main() {

    toolkit();

}

fn toolkit() {
    
    let score = 92;
    
    // ternary expression
    
    let result = if score < 80 { -1 } else { 1 };
    
    println!("Result 1: {result}");
    
    let name = "Ayhan";
    
    let result = match name {
        "Mustafa" => "My name is Mustafa",
        _ => "This is not my name!"
    };
    
    println!("Result 2: {result}");
    
    let result = {
        
        let x = 202;
        let y = 305;
        x + y
    };
    
    println!("Result 3: {result}");
    
}

// Result 1: 1
// Result 2: This is not my name!
// Result 3: 507
