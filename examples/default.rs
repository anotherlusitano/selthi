use selthi::Select;

fn main() {
    let options: Vec<&str> = vec![
        "Rust",
        "C",
        "C++",
        "Javascript",
        "Java",
        "C#",
        "Python",
        "Haskell",
        "Lisp",
        "HTML",
    ];

    let ans: Option<&str> =
        Select::new("What's your favorite programming language?", options).prompt();

    match ans {
        Some(language) => println!("{} rocks!", language),
        None => println!("There was an error, please try again"),
    }
}
