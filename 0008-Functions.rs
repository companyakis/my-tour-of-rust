fn main() {

    let x = 17;
    let y = -12;
    
    println!("x + y = {}", add(x, y));
    println!("x - y = {}", subtract(x, y));
    println!("x * y = {}", multiply(x, y));
    println!("x / y = {}", divide(x, y));

}

fn add(a: i128, b: i128) -> i128 {
    a + b
}

fn subtract (a: i128, b: i128) -> i128 {
    a - b
}

fn multiply(a: i128, b: i128) -> i128 {
    a * b
}

fn divide(a: i128, b: i128) -> f64 {
    a as f64 / b as f64
}

// x + y = 5
// x - y = 29
// x * y = -204
// x / y = -1.4166666666666667
