use std::f64;
use std::io;


fn main() {
    setup_menu();
    loop {
        println!("Please input either 1 or 2 now.");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Please make a choice.");
        let choice:u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not even a numer! Please enter a valid choice.");
                continue
            }
        };

        match choice {
            1 => { 
                calculate_kinetic_energy();
                break
            },
            2 => {
                calculate_potential_energy();
                break
            },
            _ => {
                println!("This wasn't a valid choice");
                continue
            }
        }

    }
}

fn setup_menu() {
    println!();
    println!();
    println!("What type of energy would you like to calculate?");
    println!("  1) Kinetic");
    println!("  2) Potential");
}

fn calculate_potential_energy() {
    println!();
    println!("You chose potential energy");
    println!();
}

fn calculate_kinetic_energy() {
    println!();
    println!("Now calculating kinetic energy.");
    println!();
    let force:f64 = get_value(String::from("force in Newtons"));
    let distance:f64 = get_value(String::from("distance in metres"));
    let relative_direction: f64 = get_value(String::from("relative direction in degrees"));

    let work_done = force * distance * relative_direction.to_radians().cos();
    println!("=======================");
    println!("Work done: {}j", work_done);
    println!("=======================");
}

fn get_value(name: String) -> f64 {
    loop {
        let mut value = String::new();
        println!("Please input the {}: ", name);
        io::stdin().read_line(&mut value)
            .expect("Please input a number.");
        
        let value:f64 = match value.trim().parse() {
            Ok(value) => value,
            Err(_) => { 
                println!("That wasn't a number.");
                continue
            }
        };
        break value
    }
}