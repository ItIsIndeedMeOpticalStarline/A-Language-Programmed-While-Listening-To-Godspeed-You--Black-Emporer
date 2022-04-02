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

    println!("");

    pause();
}

use std::io::prelude::*;
fn pause() 
{
    write!(std::io::stdout(), "Press any key to continue...");
    std::io::stdout().flush().unwrap();
    let _i = std::io::stdin().read(&mut [0u8]).unwrap();    
}