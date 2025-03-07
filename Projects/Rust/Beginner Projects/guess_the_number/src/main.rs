// All our imports
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use inquire::Select;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::size,
};

fn display_proximity_line(guess: u32, secret_number: u32) {
    // Merge get_terminal_width logic:
    let terminal_width: usize = match size() {
        Ok((width, _)) if width > 10 => (width - 10) as usize,
        _ => 80, // fallback if size can’t be determined or width is too small
    };

    // Merge calculate_proximity logic:
    let max_distance: u32 = 100;
    let distance: u32 = if guess > secret_number {guess - secret_number} else {secret_number - guess};

    // Raw percentage (higher percentage = closer)
    let raw_proximity: f32 = (max_distance.saturating_sub(distance) as f32 / max_distance as f32) * 100.0;

    // For incorrect guesses, quantize the percentage (floor to the nearest 10)
    let displayed_proximity: f32 = if guess != secret_number {
        let quantized: f32 = (raw_proximity / 10.0).floor() * 10.0;
        if quantized >= 100.0 {90.0} else {quantized}
    } else {
        raw_proximity
    };
    
    // Determine how many “filled” characters based on proximity:
    let filled_length: usize = ((displayed_proximity / 100.0) * terminal_width as f32) as usize;
    let empty_length: usize = terminal_width.saturating_sub(filled_length);

    // Merge proximity_to_color logic:
    let color: Color = if guess == secret_number {
        Color::Green
    } else if displayed_proximity >= 80.0 {
        Color::Red
    } else if displayed_proximity >= 60.0 {
        Color::DarkRed
    } else if displayed_proximity >= 40.0 {
        Color::Yellow
    } else if displayed_proximity >= 20.0 {
        Color::DarkBlue
    } else {
        Color::Blue
    };

    let filled = "█".repeat(filled_length);
    let empty = "-".repeat(empty_length);

    // Output the colored filled portion, then the empty portion, and the proximity percentage:
    execute!(
        io::stdout(),
        SetForegroundColor(color),
        Print(&filled),
        ResetColor,
        Print(&empty)
    )
    .unwrap();

    println!(" {:.2}%", displayed_proximity);
}

#[derive(Debug)]
enum Difficulty {
    Easy, // Easy unlimited guesses, message history, 
    Medium, // unlimted guesses, no message history
    Hard, // 5 guesses, no message history and no proximity
}

fn main() {
    println!("Welcome to the Enhanced Guess the Number game!");

    // Difficulty selection
    let options: Vec<&str> = vec!["Easy", "Medium", "Hard"];
    let difficulty: Result<&str, inquire::InquireError> = Select::new("Choose a difficulty level:", options).prompt();

    let difficulty_enum =  match difficulty {
        Ok("Easy") => Difficulty::Easy,
        Ok("Medium") => Difficulty::Medium,
        Ok("Hard") => Difficulty::Hard,

        _ => {
            println!("Invalid Selection.");
            return;
        }
    };

    println!("You selected: {:?}", difficulty_enum); //debuging - remove for gameplay

    // Number generation 
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number); // debugging - remove for gameplay

   

    loop {
        println!("Please enter your guess:");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("Please guess a number between 1 and 100.");
            continue;
        }

        display_proximity_line(guess, secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it! Congratulations!");
                break;
            }
        }
    }
}