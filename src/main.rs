use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome, please guess a number!");
    
    //Uses rand::Rng to generate a number between 1 and 100 inclusive
    let answ = rand::thread_rng().gen_range(1..=100);
    
    //loops until a correct guess is made, getting an input from 
    //the user in each loop using std::io
    loop{
        let mut g = String::new();
        io::stdin().read_line(&mut g).expect("Failed to receive input.");
        let g: i32 = g.trim().parse().expect("Please type a number!");

        println!("Your guess was: {}", g);

        //uses "match" to give the appropriate output to the user
        //depending on their answer
        match g.cmp(&answ){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it! You win!");
                break;
            }
        }
    }
}
