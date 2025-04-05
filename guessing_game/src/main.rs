use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    //thread_rng gives the current thread's local RNG
    //gen_range generates a random number in the range given as the argument
    //range syntax: lower_incl..=upper_incl
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess.");

        //variables are immutable by default
        //new() is an associated function of String
        //we don't need to specify the type of guess because of type inference
        let mut guess = String::new();

        io::stdin() // io::stdin() returns an instance of std::io::Stdin
            .read_line(&mut guess) //we pass the string we want to write the result to
            //references are immutable by default so we need the mut keyword
            .expect("Failed to read line"); //read_line() returns a Result enum value, which has a method expect()
                                            //expect returns the expected value if Result.Ok, fails with the argument string if Result.Err

        //reassign guess from the user-inputed String to an u32 parsed from that string
        //we need to annotate the type of guess explicitly for parse() to do the conversion properly
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a number dummy");
                continue;
            }
        };

        //the println! accepts macros with formats println!("{}: {}",arg1, arg2) and println!({variable_name})
        println!("Your guess was: {}", guess);

        //we run use the integer.cmp() method which returns an Ordering to compare the guess and the secret_number
        //we then match the ordering to the desired behaviour
        //match is evaluated line-by-line so if there are overlapping conditions, the first one is used
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too small......".blue().dimmed()),
            Ordering::Greater => println!("{}", "TOO BIG D:".red().underline()),
            Ordering::Equal => {
                println!("{}", "  Youre  Winer  !".green().bold().italic());
                break;
            }
        }
    }
}
