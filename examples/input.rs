use selthi::Input;

fn main() {
    let ans: Option<String> = Input::new("What's your name?").prompt();

    match ans {
        Some(name) => println!("Hello {}!", name),
        None => println!("There was an error, please try again"),
    }
}
