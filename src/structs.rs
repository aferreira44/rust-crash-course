// Structs - Used to create custom data types

// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last_name: &str) {
        self.last_name = last_name.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.red = 200;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let mut c = Color(255, 0, 0);

    // c.0 = 200;

    // println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Mary", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    println!("Person Tuple: {:?}", p.to_tuple());
}
