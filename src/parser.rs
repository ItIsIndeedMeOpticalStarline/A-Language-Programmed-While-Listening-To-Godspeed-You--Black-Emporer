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
    lines: u64,
    call: Vec<interpreter::Command>
}

pub fn parse(tokens: Vec<lexer::Token>) -> Parsed
{
    let mut result: Parsed = Parsed{lines:1, call:Vec::new()};
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
            error_message(ErrorType::UNXPCT, &tokens[idx].value, result.lines)
        }
        expect.clear();

        match &tokens[idx].key
        {
            lexer::GybeTkn::ILLEGL =>
            {
                error_message(ErrorType::ILLEGL, &tokens[idx].value, result.lines)
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
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::CMP});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "difference" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::DIF});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "dup" => // PEEK 1 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::DUP});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "end" => // NONE
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::END});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "give" => // NONE
                    {
                        expect.push(lexer::GybeTkn::KEYWRD);
                    }
                    "loop" => // PEEK 1
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::LOP});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "print" => // PEEK 1 POP 1
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::PRT});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "product" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::PCT});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "quotient" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::QUT});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "read" => // PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::RED});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "remainder" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::REM});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "skip" | "sub" => // ARG 1
                    {
                        expect.push(lexer::GybeTkn::LTERAL);
                    }
                    "sum" => // PEEK 2 POP 2 PUSH 1
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::SUM});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "swap" => // PEEK 2 POP 2 PUSH 2
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::SWP});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    "up" => // NONE
                    {
                        result.call.push(interpreter::Command{arg: Vec::new(), func: interpreter::FuncTypes::PED});
                        expect.push(lexer::GybeTkn::SPRATR);
                    }
                    _ =>
                    {
                        error_message(ErrorType::UNKNWN, &tokens[idx].value, result.lines);

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
                while tokens[idx - go_back + go_fore].value != "﹔"
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
                                }
                                val.push(*chr.first().expect("parser chr.first() function failed!") as u8);
                            }
                            "charms" | "sub" =>
                            {
                                val.push(tokens[idx - go_back + go_fore].value.parse::<u8>().expect("parser parse::<u8>() function failed!"))
                            }
                            _ =>
                            {
                                error_message(ErrorType::UNKNWN, &tokens[idx - go_back + go_fore].value, result.lines)
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

                        error_message(ErrorType::UNXPCT, &tokens[idx + go_fore + 1].value, result.lines)
                    }
                    else if tokens[idx - go_back + go_fore].value == "\n" // Stil incrememnt lines because arrays can have line breaks
                    {
                        result.lines += 1;
                    }
                    else
                    {
                        error_message(ErrorType::UNXPCT, &tokens[idx + go_fore].value, result.lines);
                    }

                    go_fore += 1;

                    if idx - go_back + go_fore >= tokens.len() // Stop the loop from going out of bounds if some maniac didn't put a ﹔
                    {
                        let error_string: String = String::from("﹔"); // Missing errors feed the missing symbol instead of the problem symbol
                        error_message(ErrorType::MISSNG, &error_string, result.lines); 
                        break;
                    }
                }

                if val.is_empty() // If the array/var has no value something's wrong
                {
                    let error_string: String = String::from("");
                    error_message(ErrorType::NOVALU, &error_string, result.lines);
                }

                if tokens[idx - go_back].value == "bite"
                {
                    result.call.push(interpreter::Command{arg: val, func: interpreter::FuncTypes::BIT});
                }
                else if tokens[idx - go_back].value == "charms"
                {
                    result.call.push(interpreter::Command{arg: val, func: interpreter::FuncTypes::CRM});
                }
                else if tokens[idx - go_back].value == "skip"
                {
                    result.call.push(interpreter::Command{arg: val, func: interpreter::FuncTypes::SKP});
                }
                else // No need for else if because we know if it's not any of those it's sub
                {
                    result.call.push(interpreter::Command{arg: val, func: interpreter::FuncTypes::SUB});
                }
                
                idx = idx - go_back + go_fore - 1; // Catch up from interior loop (-1 to stop before ﹔)
                expect.push(lexer::GybeTkn::SPRATR);
            }
            lexer::GybeTkn::SPRATR =>
            {
                match tokens[idx].value.as_str() // newlines are handled before the expect.clear()
                {
                    "﹔" =>
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