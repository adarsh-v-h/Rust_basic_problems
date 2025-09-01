use std::io;
fn main(){
    println!("Hello friend");
    // Converting temperature between fahrenheit and celsius.
    // formula is: F = (C*1.8) + 32, C = (F-32)/1.8
    println!("Do you have your values in Celsius or Fahrenheit? ");
    println!("Reply F for Fahrenheit and C for Celsius");
    let mut initial = String::new(); // asking what value they have its in Fahrenheit or celsius i.e F or C

    io::stdin() // taking the input
        .read_line(&mut initial)
        .expect("Invalid input");
    println!("Enter the value of {}", initial.trim()); // need to pass initial as an argument, and should add trim because the read_line add a new line.
    let mut value = String::new(); // to take input of the value they mentioned they have above in Fahrenheit or Celsius
    io::stdin()
        .read_line(&mut value)
        .expect("Invalid input");
    let value: f32 = value.trim().parse().expect("Invalid input, must be a number only."); // converting that string value to f32 type
    let result:f32; // a variable to hold the result of f32 type
    match initial.trim(){ // initial value is string, we add .trim() to make it &str
        "F" => {
            result = (value-32.0)/1.8;
            println!("The result is {result} C");
            },
        "C" =>{
            result = (value*1.8)+32.0;
            println!("The result is {result} F");
        },
        _ => println!("Invalid input, must be C or F")
    };

}