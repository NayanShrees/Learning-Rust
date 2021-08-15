use std::io;

fn main() {
    loop {
        let conversion = loop {
            let mut conversion = String::new();
            println!(
                "Convert Celsius to Fahrenheit (\"C\") or Fahrenheit to Celsius (\"F\") or Exit?"
            );
            io::stdin()
                .read_line(&mut conversion)
                .expect("Failed to read line");
            match conversion.as_str().trim() {
                "C" => break "C",
                "F" => break "F",
                "Exit" => std::process::exit(1),
                _ => {}
            };
        };

        let temperature = loop {
            let mut temperature = String::new();
            println!("Enter a temperature:");
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read input");
            match temperature.trim().parse::<f64>() {
                Ok(temperature) => break temperature,
                Err(_) => continue,
            };
        };

        if conversion == "C" {
            println!("{}", ((temperature * 9 as f64 / 5 as f64) + 32 as f64));
        } else if conversion == "F" {
            println!("{}", ((temperature - 32 as f64) * 5 as f64 / 9 as f64));
        } else {
            println!("Error");
        }
    }
}
