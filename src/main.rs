use std::io;
use rand::Rng;

fn create_guess(){
    let mut rng = rand::thread_rng();
    let new_guess:i32 = rng.gen_range(0..10);
    return new_guess;
}

fn find_diff(actual:i32, guess:i32){
    let diff:i32 = actual - guess;
    let diff = diff.abs();
    return diff;
}

fn main() {
    
    let crct_guess:i32 = create_guess();
    println!("Guess the number!");
    println!("Please input your guess. Please make sure it is within 1 to 10.");
    
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed");
    let guess:i32 = line.trim().parse::<i32>().unwrap();
    let diff = find_diff(crct_guess, guess);

    if guess != crct_guess{
        println!("Difference: {diff}");
        println!("Guessed Number: {guess}");
        println!("Tough Luck! Better luck next time!");
        }
    else{
        println!("Actual Number: {crct_guess}");
        println!("Guessed Number: {guess}");
        println!("Congratulations!! You have guessed correctly!!");
    }

}


