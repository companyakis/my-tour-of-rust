fn main() {

    let x = 12;
    let y = 145;
    
    let result = sum_or_multiply(x, y);
    
    println!("x + y = {}", result.0);
    
    println!("x * y = {}", result.1)

}

fn sum_or_multiply(a: i128, b: i128) -> (i128, i128) {
    
    (a + b, a * b)
}

// x + y = 157
// x * y = 1740
