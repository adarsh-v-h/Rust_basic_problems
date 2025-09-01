use std::io;
fn main(){
    println!("Hello, firend");
    // Generating Fibonacci numbers
    println!("enter the lenght of the Fibonacci series");
    let mut num = String::new(); // a variable to store the lenght of the series needed in string type
    io::stdin() // taking input
        .read_line(&mut num)
        .expect("Invalid input");
    let num:u32 = num.trim().parse().expect("Invalid input"); // converting the string to u32
    println!("The number given is {num}");
    let mut a = 0;
    let mut b =1;
    println!("The fibonaic series of lenght {}", num);
    for _ in 0..num{
        print!("{} ", a); // using print! to void going to the next line.
        let next = a +b;
        a = b;
        b = next;
    }
    println!();// to get a line space


}