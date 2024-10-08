fn main() {

    let years: [u16; 5] = [1990, 2020, 2022, 2024, 2025];
    
    println!("Years: {:?}", years);
    
    println!("Last year is {}.", years[years.len() - 1])

}

// Years: [1990, 2020, 2022, 2024, 2025]
// Last year is 2025.
