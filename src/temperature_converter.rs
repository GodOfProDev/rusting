use crate::utils;

pub fn temperature_convertor() {
    loop {
        println!("Enter the unit you are using: ");

        let unit = utils::get_string();
        let unit = unit.chars().nth(0).expect("Failed to read the input");

        println!("Enter the temperature: ");

        let temperature = utils::get_number_f32();

        if unit == 'C' || unit == 'c' {
            println!("Celsius: {temperature}\tFahrenheit: {}", (temperature * 9.0 / 5.0) + 32.0);
            break;
        } else if unit == 'F' || unit == 'f' {
            println!("Fahrenheit: {temperature}\tCelsius: {}", (temperature - 32.0) * 5.0 / 9.0);
            break;
        } else {
            println!("Invalid Unit");
        }
    }
}