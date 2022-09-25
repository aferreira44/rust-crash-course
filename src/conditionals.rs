pub fn run() {
    let age = 33;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // If/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What's would like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Get out of here!");
    } else {
        println!("Bartender: I need to see your ID!");
    }

    // Shorthand IF
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);
}
