fn main() {

    // Using a static method to create an instance of String
    
    let who_i_am = String::from("Mustafa Buyukdereli");
    
    // Using a method on the instance
    
    println!("My name '{}' has {} characters.", who_i_am, who_i_am.len()); // My name 'Mustafa Buyukdereli' has 19 characters.
}

/*

Unlike functions, methods are functions associated with a specific data type.

static methods — methods that belong to a type itself are called using the :: operator.

instance methods — methods that belong to an instance of a type are called using the . operator.

*/
