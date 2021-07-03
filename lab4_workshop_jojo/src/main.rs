use std::io;

fn main() {
    println!("Enter Value : ");
    let mut input = String::new();
    io::stdin() .read_line(&mut input).expect("Failed to read line");
    let input:u8 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => (0)
    };   
    println!("You Print : {}", input);
}
