fn main() {
    let year = 2045;
    
    match year {
    
        1980..=2000 => {
            println!("We were child!");
        }
        
        // we can bind the matched number to a variable
        
        matched_year @ 2001..=2019 => {
            println!("Year {matched_year}...");
        }
        
        2020 | 2021 | 2022 | 2023 | 2024 => {
            println!("These years...");
        }
        
        _=> {
            println!("Try again...");
        }

    }

}
