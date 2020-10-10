use std::io;
use std::string::String;

fn main() {
    println!("Welcome to My Temperature Converter!\n");

    let temperaure_options= ("Fahrenheit", "Celsius");

    println!("[1] Convert from {} to {} \n", temperaure_options.0, temperaure_options.1);
    println!("[2] Convert from {} to {} \n", temperaure_options.1, temperaure_options.0);
    let mut converted_temprature: f32;
    let mut temprature_output = temperaure_options.1; // Default to Celsius

    converted_temprature = loop {
        println!("Please choose a converter: [Enter 1 or 2] \n");
        let mut selected_converter = String::new();

        io::stdin()
        .read_line(&mut selected_converter)
        .expect("Error while getting user input");

        // Shadwoing to rescue!
        let selected_converter : u8 = match selected_converter.trim().parse() {
            Ok(num)=> num,
            Err(_) => continue,
        };

        if selected_converter != 1 &&  selected_converter != 2 {
            println!("Wrong option, the value is {}", selected_converter)
        } else {
            if selected_converter == 1 {
           
                println!("Please enter the temprature in {} [example: 100]\n", temperaure_options.0);
                let mut fahrenheit_temprature = String::new();
                io::stdin()
                .read_line(&mut fahrenheit_temprature)
                .expect("Error while getting input for fahrenheit");

                let fahrenheit_temprature = match fahrenheit_temprature.trim().parse() {
                    Ok(num)=> num,
                    Err(_) => continue,
                };
                converted_temprature = fahrenheit_to_celsius(fahrenheit_temprature);
            } else {
                println!("Please enter the temprature in {} [example: 100]\n", temperaure_options.1);
                let mut celsius_temprature = String::new();
                io::stdin()
                .read_line(&mut celsius_temprature)
                .expect("Error while getting input for Celsius");

                let celsius_temprature = match celsius_temprature.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                temprature_output = temperaure_options.0;
                converted_temprature = celsius_to_fahrenheit(celsius_temprature);
            }
            break converted_temprature;
        }
    };
    println!("Tempratue is {} in {}", converted_temprature, temprature_output);
}

fn fahrenheit_to_celsius(fahrenheit_temprature: f32) -> f32 {
     // Formula (n°F - 32) * 5 / 9 = n °C;
     return (fahrenheit_temprature - 32.0) * 5.0/9.0;
}

fn celsius_to_fahrenheit(celsius_temprature: f32) -> f32 {
     // Formula (n°C * 9/5) + 32 = n °F;
     return (celsius_temprature * 9.00/5.00) + 32.00;
}
