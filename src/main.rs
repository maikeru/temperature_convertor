use std::io;

fn main() {
    loop {
        println!("Select conversion:");
        println!("1. Fahrenheit -> Celsius");
        println!("2. Celsius -> Fahrenheit");
        println!("Enter temperature in Fahrenheit:");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read option");
        let option = option.trim();
        if option == "1" {
            let fahrenheit: f32 = get_input_temperature();
            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("That is {} in degrees Celsius", celsius);
            break;
        }
        else if option == "2" {
            let celsius: f32 = get_input_temperature();
            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("That is {} in degrees Fahrenheit", fahrenheit);
            break;
        }
    }
}

fn get_input_temperature() -> f32 {
    println!("Enter temperature to convert");
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).expect("Failed to read temperature");

    let temperature = temperature.trim();
    temperature.parse().unwrap()
}

fn celsius_to_fahrenheit(t: f32) -> f32 {
    (t / 0.5556) + 32.0
}

fn fahrenheit_to_celsius(t: f32) -> f32 {
    (t - 32.0) * 0.5556
}
