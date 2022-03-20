use std::path::Path;

mod interpreter;
mod lexer;
mod parser;

fn main() 
{
    print!("Hello and welcome to the official ALPWLTGY!BE compiler!\n");
    println!("Please enter a path to a .gybe file (Ex: C:\\Users\\Eve\\Desktop\\HelloWorld.gybe)\n");

    let mut file_in: String = String::new();
    std::io::stdin().read_line(&mut file_in).unwrap();

    let file_path: &Path = Path::new(file_in.trim());

    let contents: std::io::Result<std::string::String> = std::fs::read_to_string(file_path);
    if contents.is_err()
    {
        println!("Unable to read file {} ({})", file_path.display(), contents.unwrap_err());
        return;
    }

    let commands: parser::Parsed = parser::parse(lexer::lex(contents.unwrap()));

    // Says this even if compilation fails
    println!("\nParsed successfully! Now interpreting...\n");

    if commands.run
    {
        interpreter::interpret(commands);    
    }
}
