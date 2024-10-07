fn main() {
    // rust infers the type of x
    
    let year = 2024;
    println!("Year: {}", year);

    // rust can also be explicit about the type
    
    let normal_pi: f32 = 3.14;
    println!("{normal_pi}");

    // rust can also declare and initialize later, but this is rarely done

    let step;
    step = 101;
    println!("Step is {step}")
}

// Year: 2024
// 3.14
// Step is 101
