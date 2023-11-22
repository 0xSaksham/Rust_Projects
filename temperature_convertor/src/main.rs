use std::io;
fn main() {

    let mut celsius = String::new();

    println!("Enter Temperature to be converted: (in C) ");

    io::stdin().read_line(&mut celsius).expect("Enter a Number");

    let celsius: f64 = celsius.trim().parse().expect("Invalid Input");

    let fahrenheit = (1.8 * celsius) + 32.0;

    println!("Temp in F: {fahrenheit}");


}
