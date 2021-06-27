use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("secert number is: {}",secret_number);
    loop{
        if get_and_check(&secret_number) {
            break;
        }
        // println!("Please input your guess: ");
        // let mut guess = String::new();
        // io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        // print!("You guessed: {}",guess);
    
        // let guess: u32 = guess.trim().parse().expect("Failed type a number!");
    
        // match guess.cmp(&secret_number){
        //     Ordering::Less => println!("Too Small!"),
        //     Ordering::Greater => println!("Too Big!"),
        //     Ordering::Equal => { 
        //         println!("You Win!"); 
        //         break; 
        //     }
        // }
    }
    println!("Verygood <3");
}

fn get_and_check(secret: &u32) -> bool {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{} is not a number. Try again: ", guess.trim());
            return false;
        }
    };

    println!("You guessed: {}", guess);

    match guess.cmp(secret) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }
    false
}

//  mut Dynamic Value
//  Ordering
//  Loop
//  Match

// https://gist.github.com/ddrscott/991a329b7f1c1f7682da5e4c24cdecc5