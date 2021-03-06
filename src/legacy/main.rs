/*
Copyright (c) 2022 ItIsIndeedMeOpticalStarline

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::path::Path;

mod c_code;
mod translator;
mod lexer;

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

    let vec: std::vec::Vec<lexer::Token> = lexer::lex(contents.unwrap());

    let c_code: String = translator::translate(vec);

    println!("\n{} successfully compiled!", file_path.display());
    println!("Please enter an output path (Ex: C:\\Users\\Eve\\Desktop)\n");

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

    // This printing even if you fail is on purpose. Don't you dare delete it.
    println!("\nEverything was successful. Good work!");
}
