use std::path::Path;

mod interpreter;
mod lexer;
mod alpwltgybe;

fn main() 
{
    let file_path: &Path = Path::new("C:\\Users\\Eliot Lauster\\Desktop\\Programming\\Rust\\ALPWLTGY-BE\\src\\examples\\HelloWorld.gybe");

    let contents: std::io::Result<std::string::String> = std::fs::read_to_string(file_path);
    if contents.is_err()
    {
        println!("Unable to read file {} ({})", file_path.display(), contents.unwrap_err());
        return;
    }

    let vec: std::vec::Vec<lexer::Token> = lexer::lex(contents.unwrap());

    //for tkn in vec
    //{
        //lexer::tkn_print(tkn);
    //}

    let c_code: String = interpreter::interpret(vec);

    println!("{}", c_code.as_str());

    let result: std::io::Result<()> = std::fs::write("C:\\Users\\Eliot Lauster\\Desktop\\Programming\\Rust\\ALPWLTGY-BE\\src\\examples\\HelloWorld.cpp", c_code);
    if result.is_err()
    {
        println!("Unable to write to file {} ({})", file_path.display(), result.unwrap_err());
        return;
    }
}
