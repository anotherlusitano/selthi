use selthi::Select;

fn main() {
    let options: Vec<&str> = vec!["Linux", "Windows", "macOS"];
    let images: Vec<&str> = vec![
        "./examples/images/linux.png",
        "./examples/images/windows.png",
        "./examples/images/macos.png",
    ];

    let ans: Option<&str> = Select::new("What's your favorite operating system?", options)
        .with_images(images)
        .without_help_message()
        .prompt();

    match ans {
        Some(os) => println!("{} is a good choice!", os),
        None => println!("There was an error, please try again"),
    }
}
