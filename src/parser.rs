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

use crate::interpreter;
use crate::lexer;
use std::fmt::Write;

#[derive(Debug)]
pub enum ErrorType
{
    ILLEGL,
    MISSNG,
    NOVALU,
    UNKNWN,
    UNXPCT
}

pub struct Parsed
{
    pub call: Vec<interpreter::Command>,
    pub lines: u64,
    pub run: bool
}

pub fn parse(tokens: Vec<lexer::Token>) -> Parsed
{
    let mut result: Parsed = Parsed{call:Vec::new(), lines: 1, run: true};
    let mut expect: Vec<lexer::GybeTkn> = vec![lexer::GybeTkn::KEYWRD]; 

    let mut idx: usize = 0;
    while idx < tokens.len()
    {
        if &tokens[idx].value == "\n"
        {
            result.lines += 1;
            idx += 1;
            continue;
        }

        if !expect.contains(&tokens[idx].key)
        {
            error_message(ErrorType::UNXPCT, &tokens[idx].value, result.lines);
            result.run = false;
        }
        expect.clear();

        match &tokens[idx].key
        {
            lexer::GybeTkn::ILLEGL =>
            {
                error_message(ErrorType::ILLEGL, &tokens[idx].value, result.lines);
                result.run = false;
            }
            lexer::GybeTkn::KEYWRD =>
            {
                match tokens[idx].value.as_str()
                {
                    "bite" | "charms" => // ARG 1 PUSH 1
                    {
                        expect.push(lexer::GybeTkn::LTERAL);
                    }
                    "comp" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::CMP, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "difference" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::DIF, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "dup" => // PEEK 1 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::DUP, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "end" => // NONE
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::END, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "give" => // NONE
                    {
                        expect.push(lexer::GybeTkn::KEYWRD);
                    }
                    "loop" => // PEEK 1
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::LOP, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "print" => // PEEK 1 POP 1
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::PRT, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "product" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::PCT, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "quotient" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::QUT, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "read" => // PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::RED, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "remainder" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::REM, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "skip" | "sub" => // ARG 1
                    {
                        expect.push(lexer::GybeTkn::LTERAL);
                    }
                    "sum" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::SUM, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "swap" => // PEEK 2 POP 2 PUSH 2
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::SWP, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "up" => // NONE
                    {
                        result.call.push(interpreter::Command{arg: interpreter::Arg{elm: Vec::new(), typ: interpreter::DataTypes::NONE}, func: interpreter::FuncTypes::PED, line: result.lines});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    _ =>
                    {
                        error_message(ErrorType::UNKNWN, &tokens[idx].value, result.lines);
                        result.run = false;

                        expect.push(lexer::GybeTkn::KEYWRD);
                        expect.push(lexer::GybeTkn::LTERAL);
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                }
            }
            lexer::GybeTkn::LTERAL =>
            {
                let mut go_back: usize = 1;
                let mut go_fore: usize = 1;
                let mut val: Vec<u8> = Vec::new();

                // Find index of type keyword
                while tokens[idx - go_back].value != "bite" &&
                tokens[idx - go_back].value != "charms" &&
                tokens[idx - go_back].value != "skip" &&
                tokens[idx - go_back].value != "sub"
                {
                    go_back += 1;
                }

                // Iterate until end of potential array
                while tokens[idx - go_back + go_fore].value != "???"
                {
                    if tokens[idx - go_back + go_fore].key == lexer::GybeTkn::LTERAL
                    {
                        match tokens[idx - go_back].value.as_str()
                        {
                            "bite" | "skip" =>
                            {
                                let chr: Vec<char> = tokens[idx - go_back + go_fore].value.chars().collect();
                                if chr.len() > 1 // throw error if there is more than one char in the charms literal
                                {
                                    let val_str = chr.iter().collect();
                                    error_message(ErrorType::ILLEGL, &val_str, result.lines);
                                    result.run = false;
                                }
                                val.push(*chr.first().expect("parser chr.first() function failed!") as u8);
                            }
                            "charms" | "sub" =>
                            {
                                val.push(tokens[idx - go_back + go_fore].value.parse::<u8>().expect("parser parse::<u8>() function failed!"))
                            }
                            _ =>
                            {
                                error_message(ErrorType::UNKNWN, &tokens[idx - go_back + go_fore].value, result.lines);
                                result.run = false;
                            }
                        }
                    }
                    else if tokens[idx - go_back + go_fore].value == "." // Check for . between each element
                    {
                        if tokens[idx - go_back].value.as_str() != "skip" && // Skips and subs can't have array arguments
                        tokens[idx - go_back].value.as_str() != "sub"
                        {
                            if tokens[idx - go_back + go_fore + 1].value == "\n" ||
                            tokens[idx - go_back + go_fore + 1].key == lexer::GybeTkn::LTERAL
                            {
                                go_fore += 1;
                                continue;
                            }
                        }

                        error_message(ErrorType::UNXPCT, &tokens[idx + go_fore + 1].value, result.lines);
                        result.run = false;
                    }
                    else if tokens[idx - go_back + go_fore].value == "\n" // Stil incrememnt lines because arrays can have line breaks
                    {
                        result.lines += 1;
                    }
                    else
                    {
                        error_message(ErrorType::UNXPCT, &tokens[idx + go_fore].value, result.lines);
                        result.run = false;
                    }

                    go_fore += 1;

                    if idx - go_back + go_fore >= tokens.len() // Stop the loop from going out of bounds if some maniac didn't put a ???
                    {
                        let error_string: String = String::from("???"); // Missing errors feed the missing symbol instead of the problem symbol
                        error_message(ErrorType::MISSNG, &error_string, result.lines); 
                        result.run = false;
                        break;
                    }
                }

                if val.is_empty() // If the array/var has no value something's wrong
                {
                    let error_string: String = String::from("");
                    error_message(ErrorType::NOVALU, &error_string, result.lines);
                    result.run = false;
                }

                if tokens[idx - go_back].value == "bite"
                {
                    result.call.push(interpreter::Command{arg: interpreter::Arg{elm: val, typ: interpreter::DataTypes::BITE}, func: interpreter::FuncTypes::BIT, line: result.lines});
                }
                else if tokens[idx - go_back].value == "charms"
                {
                    result.call.push(interpreter::Command{arg: interpreter::Arg{elm: val, typ: interpreter::DataTypes::CHRM}, func: interpreter::FuncTypes::CRM, line: result.lines});
                }
                else if tokens[idx - go_back].value == "skip"
                {
                    result.call.push(interpreter::Command{arg: interpreter::Arg{elm: val, typ: interpreter::DataTypes::BITE}, func: interpreter::FuncTypes::SKP, line: result.lines});
                }
                else // No need for else if because we know if it's not any of those it's sub
                {
                    result.call.push(interpreter::Command{arg: interpreter::Arg{elm: val, typ: interpreter::DataTypes::CHRM}, func: interpreter::FuncTypes::SUB, line: result.lines});
                }
                
                idx = idx - go_back + go_fore - 1; // Catch up from interior loop (-1 to stop before ???)
                expect.push(lexer::GybeTkn::SPRATR);
            }
            lexer::GybeTkn::SPRATR =>
            {
                match tokens[idx].value.as_str() // newlines are handled before the expect.clear()
                {
                    "???" =>
                    {
                        expect.push(lexer::GybeTkn::KEYWRD);
                    }
                    "." =>
                    {
                        expect.push(lexer::GybeTkn::LTERAL);
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    _ =>
                    {
                        error_message(ErrorType::UNKNWN, &tokens[idx].value, result.lines);
                        result.run = false;
                    }
                }
            }
        }
        idx += 1;
    }

    return result;
}

fn error_message(error_type: ErrorType, value: &String, line: u64)
{
    let mut error_string: String = String::new();
    match error_type
    {
        ErrorType::ILLEGL =>
        {
            write!(error_string, "Illegal token with value {} found", value).expect("parser ILLEGL write! macro failed.");
        }
        ErrorType::MISSNG =>
        {
            write!(error_string, "Missing a token with the value {}", value).expect("parser MISSNG write! macro failed.");
        }
        ErrorType::NOVALU =>
        {
            write!(error_string, "Expected a token with a value, found NULL").expect("parser NOVALU write! macro failed.");
        }
        ErrorType::UNKNWN =>
        {
            write!(error_string, "Unknown token with value {} found", value).expect("parser UNKNWN write! macro failed.");
        }
        ErrorType::UNXPCT =>
        {
            write!(error_string, "Unexpected token with value {} found", value).expect("parser UNXPCT write! macro failed.");
        }
    }

    print!("ERROR: {:?} ({}), on line {}. Maybe this language isn't meant for you?\n", error_type, error_string, line);
}