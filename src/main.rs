// Defining the functions

fn c_to_f(celsius: f32) -> f32 {
    return (celsius * 9.0 / 5.0) + 32.0;
}
fn c_to_k(celsius: f32) -> f32 {
    return celsius + 273.15;
}
fn f_to_c(fahrenheit: f32) -> f32 {
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}
fn f_to_k(fahrenheit: f32) -> f32 {
    return f_to_c(fahrenheit) + 273.15;
}
fn k_to_c(kelvin: f32) -> f32 {
    return kelvin - 273.15;
}
fn k_to_f(kelvin: f32) -> f32 {
    return c_to_f(k_to_c(kelvin));
}

fn main() {
    // Main program loop
    loop {
        println!("TempConverter.rs");
        println!("What would you like to convert?");
        println!("0. Exit");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Celsius to Kelvin");
        println!("4. Kelvin to Celsius");
        println!("5. Fahrenheit to Kelvin");
        println!("6. Kelvin to Fahrenheit");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number");
        // Converting between units
        if choice == 1 {
            println!("Enter temperature in Celsius");
            let mut temp: String = String::new();
            std::io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");
            let temp: f32 = temp.trim().parse().expect("Please type a number");
            let fahrenheit = c_to_f(temp);
            println!("{}°C = {}°F", temp, fahrenheit);
        //
        } else if choice == 2 {
            println!("Enter temperature in Fahrenheit");
            let mut temp: String = String::new();
            std::io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");
            let temp: f32 = temp.trim().parse().expect("Please type a number");
            let celsius = f_to_c(temp);
            println!("{}°F = {}°C", temp, celsius);
        //
        } else if choice == 3 {
            println!("Enter temperature in Celsius");
            let mut temp: String = String::new();
            std::io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");
            let temp: f32 = temp.trim().parse().expect("Please type a number");
            let kelvin = c_to_k(temp);
            println!("{}°C = {}K", temp, kelvin);
        //
        } else if choice == 4 {
            println!("Enter temperature in Kelvin");
            let mut temp: String = String::new();
            std::io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");
            let temp: f32 = temp.trim().parse().expect("Please type a number");
            let celsius = k_to_c(temp);
            println!("{}K = {}°C", temp, celsius);
        //
        } else if choice == 5 {
            println!("Enter a temperature in Fahrenheit");
            let mut temp: String = String::new();
            std::io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");
            let temp: f32 = temp.trim().parse().expect("Please type a number");
            let kelvin = f_to_k(temp);
            println!("{}°F = {}K", temp, kelvin);
        //
        } else if choice == 6 {
            println!("Enter a temperature in Kelvin");
            let mut temp: String = String::new();
            std::io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");
            let temp: f32 = temp.trim().parse().expect("Please type a number");
            let fahrenheit = k_to_f(temp);
            println!("{}K = {}°F", temp, fahrenheit);
        //
        } else if choice == 0 {
            println!("Exiting...");
            break;
        //
        } else {
            println!("Invalid choice! Please enter a value within the range.");
        }
    }
}
