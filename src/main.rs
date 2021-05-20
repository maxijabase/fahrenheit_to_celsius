use std::io;

fn main() {
    loop {
        println!("Insert your degrees to convert to Celsius! Insert a double zero to exit.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Provide a number!");

        let degrees: i32 = input.trim().parse().expect("Insert a number!!!");

        if degrees == 00 {
            break;
        }

        println!(
            "{}F are equivalent to {}C",
            &degrees,
            fahrenheit_to_celsius(degrees).round()
        );
    }
}

fn fahrenheit_to_celsius(degrees: i32) -> f64 {
    let degrees: f64 = degrees as f64;
    (degrees - 32.0) * (5.0 / 9.0)
}
