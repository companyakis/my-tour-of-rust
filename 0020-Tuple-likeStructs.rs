struct Coordinates(i32, i32, i32); // x, y, z

struct Employee(String, String, u16); // name, department, id

fn main() {

    let player_coordinates = Coordinates(22, 34, 15);
    
    println!("Player coordinates: {} - {} - {}", player_coordinates.0, player_coordinates.1, player_coordinates.2);
    
    let emp_mustafa = Employee("Mustafa".to_string(), "FinTech".to_string(), 101);
    
    println!("Name: {} - department: {} - ID: {}", emp_mustafa.0, emp_mustafa.1, emp_mustafa.2);

}

// Player coordinates: 22 - 34 - 15
// Name: Mustafa - department: FinTech - ID: 101
