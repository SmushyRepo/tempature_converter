use std::io;
use std::process;
fn main() {
	println!("Welcome to my tempature converter!");
	loop {
		let mut choice = String::new();
		println!("\nSelect the tempature you are going to input to get the converted outcome by writing one of the numbers or one of the symbols!");
		println!("1. Fahrenheit (°F)");
		println!("2. Celsius (°C)");
		println!("3. Kelvin (K)");
		println!("4. Exit Program");
		io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
			eprintln!("Application Error: {}", err);
			process::exit(1);
		});
		let choice = choice.trim() as &str;
		match choice {
			"1" | "1." | "°F" | "°f" | "F" | "f" => {
				let choice = convert();
				println!("{} degrees Fahrenheit!", choice);
				println!("{} degrees Celsius!", (choice - 32.0) * 5.0/9.0);
				println!("{} Kelvin!", (choice - 32.0) * 5.0/9.0 + 273.15);
			}
			"2" | "2." | "°C" | "°c" | "C" | "c" => {
				let choice = convert();
				println!("{} degrees Celsius!", choice);
				println!("{} degrees Fahrenheit!", (choice * 9.0/5.0) + 32.0);
				println!("{} Kelvin!", choice + 273.15);
			}
			"3" | "3." | "K" | "k" => {
				let choice = convert();
				println!("{} Kelvin!", choice);
				println!("{} degrees Fahrenheit!", (choice - 273.15) * 9.0/5.0 + 32.0);
				println!("{} degrees Celsius!", choice - 273.15);
			}
			"4" | "4." => {
				println!("Goodbye!");
				process::exit(0);
			}
			&_ => {
				eprintln!("You entered an invalid value.");
				process::exit(1);
			}
		}
	}
	
}
fn convert()->f64 {
	println!("Enter the value you want to convert.");
	let mut choice = String::new();
	io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
		eprintln!("Application Error: {}", err);
		process::exit(1);
	});
	let choice:f64 = choice.trim().parse().unwrap_or_else(|err| {
		eprintln!("Application Error: {}", err);
		process::exit(1);
				});
	println!("The value you entered is:");
	return choice;
}
