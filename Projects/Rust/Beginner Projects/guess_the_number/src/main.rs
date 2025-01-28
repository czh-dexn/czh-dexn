use std::io;
use rand::Rng;
use std::cmp::Ordering;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::size,
};

fn get_terminal_width() -> usize {
    match size() {
        // `size()` returns a Result<(u16, u16)>, so pattern-match on the tuple:
        Ok((width, _height)) => {
            // Subtract some padding to avoid filling the entire line
            if width > 10 {
                (width - 10) as usize
            } else {
                80
            }
        }
        Err(_) => 80, // Fallback width if we fail to get terminal size
    }
}

fn calculate_proximity(guess: u32, secret_number: u32) -> f32 {
    let max_distance = 100;
    let distance = if guess > secret_number {
        guess - secret_number
    } else {
        secret_number - guess
    };
    // Higher “distance” => lower “proximity”
    ((max_distance - distance) as f32 / max_distance as f32) * 100.0
}

fn proximity_to_color(proximity: f32) -> Color {
    if proximity >= 80.0 {
        Color::Red
    } else if proximity >= 60.0 {
        Color::DarkRed
    } else if proximity >= 40.0 {
        Color::Yellow
    } else if proximity >= 20.0 {
        Color::DarkBlue
    } else {
        Color::Blue
    }
}

fn display_proximity_line(proximity: f32) {
    let terminal_width = get_terminal_width();
    // Determine how many “filled” characters to draw based on proximity
    let filled_length = ((proximity / 100.0) * terminal_width as f32) as usize;
    let empty_length = terminal_width.saturating_sub(filled_length);

    let color = proximity_to_color(proximity);
    let filled = "█".repeat(filled_length);
    let empty = "-".repeat(empty_length);

    execute!(
        io::stdout(),
        SetForegroundColor(color),
        Print(&filled),
        ResetColor,
        Print(&empty),
    )
    .unwrap();

    println!(" {:.2}%", proximity);
}

fn main() {
    println!("Welcome to the Enhanced Guess the Number game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess:");
        let mut guess = String::new();

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

        let proximity = calculate_proximity(guess, secret_number);
        display_proximity_line(proximity);

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