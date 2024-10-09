fn main() {

    let mut salary_usd = 3000;
    
    let result = loop {
    
        salary_usd += 100;
        
        if salary_usd == 6000 {
            
            break "I am happy now:)";
        }
    };
    
    println!("My salary: ${salary_usd}  and I say {}", result); // My salary: $6000  and I say I am happy now:)
    
}
