use std::path::Path;

mod c_code;
mod translator;
mod lexer;

fn main() 
{
    print!("Hello and welcome to the official ALPWLTGY!BE compiler!\n");
    println!("Please enter a path to a .gybe file (Ex: C:\\Users\\Jackie\\Desktop\\HelloWorld.gybe)\n");

    let mut file_in: String = String::new();
    std::io::stdin().read_line(&mut file_in).unwrap();

    let file_path: &Path = Path::new(file_in.trim());

    let contents: std::io::Result<std::string::String> = std::fs::read_to_string(file_path);
    if contents.is_err()
    {
        println!("Unable to read file {} ({})", file_path.display(), contents.unwrap_err());
        return;
    }

    let vec: std::vec::Vec<lexer::Token> = lexer::lex(contents.unwrap());

    let c_code: String = translator::translate(vec);

    println!("\n{} successfully compiled!", file_path.display());
    println!("Please enter an output path (Ex: C:\\Users\\Jackie\\Desktop)\n");

    let mut file_out: String = String::new();
    std::io::stdin().read_line(&mut file_out).unwrap();
    file_out = file_out.trim().to_string();

    let file_name: &str = file_path.file_stem().unwrap_or(&std::ffi::OsStr::new("")).to_str().unwrap_or("");

    let mut cpp_path: String = file_out.clone();
    cpp_path.push_str("\\");
    cpp_path.push_str(file_name);
    cpp_path.push_str(".cpp");

    let result: std::io::Result<()> = std::fs::write(&cpp_path, c_code);
    if result.is_err()
    {
        println!("Unable to create file at {} ({})", cpp_path, result.unwrap_err());
        return;
    }

    /*let mut cpp: String = file_name.to_string();
    cpp.push_str(".cpp");

    let mut exe: String = file_name.to_string();
    exe.push_str(".exe");
    
    if cfg!(target_os = "windows")
    {
        std::process::Command::new("cd").arg(file_out);
    }
    else
    {
        std::process::Command::new("sh").arg(file_out);
    }

    std::process::Command::new("cl").arg("/EHsc").arg(cpp);
    std::process::Command::new(exe);*/

    println!("\nEverything was successful. Good work!");
}
