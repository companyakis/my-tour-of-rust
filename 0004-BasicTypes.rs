fn main() {
    let month = 10; // by default this is i32
    let year = 2024u16;
    let pi = 3.14; // by default this is f64
    let pi2 = 3.14f32;
    let first_letter: char = 'a';
    let ferris = '🦀'; // also a unicode character
    let ready: bool = true;
    let info = (101, true);
    let sentence = "hello world!";
    let greeting = "hi there!";

    
    println!("{month}, {year}, {pi}, {pi2}, {first_letter}"); 
    
    println!("{}", ferris);
    
    println!("{ready}, {sentence}, {greeting}");
    
    println!("{} - {}", info.0, info.1);
}

// 10, 2024, 3.14, 3.14, a
// 🦀
// true, hello world!, hi there!
// 101 - true
