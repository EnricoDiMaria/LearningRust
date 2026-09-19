use std::io; //io library from standard (std)
use std::cmp::Ordering; //Ordering is an enum (a type) that is the result of a comparation between two numbers
use rand::Rng; //to use a dependencie you first have to define it in Cargo.toml and then run `cargo build`
use colored::*;

fn main() {
    println!("Guess the number!");

    //let = a variable (standard variable is immutable)
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is {}", secret_number);

    loop {
        println!("Enter your input:");

        //variable to store the user input (mut = mutable variable, let is used to define a variable)
        let mut guess = String::new(); //String is a type in Rust standard library and is a growable string
                                        //new() returns an empty string that we can use
                                        //:: signals that the function is called directly on the type string, non the instance guess (associated functions/static methods)
                                        //:: is also used to access modules from libraries

        io::stdin() //it returns an object of type Stdin
            .read_line(&mut guess) //it takes as a parameter a mutable reference to an object without taking ownership *
            .expect("Failed to read line!");
        //the singles dots are used to call a method on the previous object, so read_line on the objecy Stdin and expect on the risult of read_line
        //read_line returns a type Result, that returns an OK(...) or and error: expect is used to print something in case an error occurs

        //shadowing: used to change type to a variable (to use the same name to create a new variable that replaces the old one with the same name)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //in the OK case the value is a num (u32) and is returned to guess
            Err(_) => continue, //with every error (_) it continues with the next iteration of the loop (from the start of the loop)
        }; //parse makes a conversion to the type specified after the name of the variable with :

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { //to match every possible result with something else
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
}

// * In Rust every value has one an only owner, so the function can only modify the content of the variable by reference, 
// but cannot take ownership. Ownership is important because when the owner exits the scope, the variable is destroyed 
//(so no memory leaks or garbage collector problems)