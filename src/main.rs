use std::f64;
use std::io;


fn main() {
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