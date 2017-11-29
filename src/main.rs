use std::f64;
use std::io;


fn main() {
    setup_menu();
    loop {
        println!("Please input either 1, 2 or 3 now.");
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
            3 => { 
                calculate_work_done();
                break
            }
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
    println!("  3) Work done");
}

 fn calculate_kinetic_energy() {
    println!();
    println!();
    println!("You are calculating kinetic energy.");
    let velocity = get_value(String::from("velocity in m/s^2"));
    let mass = get_value(String::from("mass in kg"));

    let kinetic_energy:f64 = 0.5 * velocity * mass * mass;
    println!("=======================");
    println!("Kinetic Energy: {:.8}j", kinetic_energy);
    println!("=======================");
 }

fn calculate_potential_energy() {
    println!();
    println!();
    println!("You chose potential energy");
    println!("Are you calculating potential energy due to a elasticity or gravity?");
    println!("  1) Elastic");
    println!("  2) Gravity");
    loop {
        println!("Please enter 1 or 2 now");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Not a number. Please type 1 or 2");
        let choice = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number. Please enter 1 or 2");
                continue
            }
        };
        match choice {
            1 => {
                calculate_spring_energy();
                break
            },
            2 => {
                calculate_gravitational_energy();
                break
            },
            _ => {
                println!("Not a valid choice.");
                continue
            }
        }
    }
}

fn calculate_gravitational_energy() {
    println!("You are calculating gravitational potential energy");
    println!("Are you on earth? (Y/n)");
    let is_on_earth:bool = get_yes_no_value();
    let gravity:f64 = if is_on_earth {
        9.81
    } else {
        get_value(String::from("strength of gravity in N"))
    };
    let height = get_value(String::from("height of the object, in meters"));
    let mass = get_value(String::from("mass of the object in kg"));

    let gravitational_potential_energy: f64 = mass * gravity * height;
    println!("=======================");
    println!("Gravitational Potential Energy: {:.8}j", gravitational_potential_energy);
    println!("=======================");
}

fn calculate_spring_energy() {
    println!("You are calculating elastic potential energy");
    let constant = get_value(String::from("spring constant"));
    let displacement = get_value(String::from("displacement in meters"));

    let elastic_potential_energy:f64 = 0.5 * constant * displacement * displacement;

    println!("=======================");
    println!("Elastic Potential Energy: {:.8}j", elastic_potential_energy);
    println!("=======================");
}

fn calculate_work_done() {
    println!();
    println!("Now calculating work done.");
    println!();
    let force:f64 = get_value(String::from("force in Newtons"));
    let distance:f64 = get_value(String::from("distance in metres"));
    let relative_direction: f64 = get_value(String::from("relative direction in degrees"));

    let work_done = force * distance * relative_direction.to_radians().cos();
    println!("=======================");
    println!("Work done: {:.8}j", work_done);
    println!("=======================");
}
// ==============
// Helper methods
// ==============

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

fn get_yes_no_value() -> bool {
    loop {
        let mut is_on_earth = String::new();
        io::stdin().read_line(&mut is_on_earth)
            .expect("Please enter Y or n");
        match is_on_earth.trim().as_ref() {
            "Y" => return true,
            "n" => return false,
            _ => {
                println!("Please enter Y or n");
                continue
            }
        };
    };
}