fn main() {
    println!("Welcome to the temperature converter!");

    // Giving the options to start from 
    let options: [&str; 3] = ["Celcius","Fahrenheit", "Kelvin, Exit"]; 

    // prompting the user for their input
    let question = Question {
        question_type: "list",
        message: "What operation would you like to perform?",
        name: "operation",
        choices: options,
    };
}
