use crate::alpwltgybe;
use crate::lexer;
use std::fmt::Write;

// This is really messy if someone wants to clean it up go ahead but that's not gonna be my job

pub fn interpret(tokens: Vec<lexer::Token>) -> String
{
    let mut program: String = String::new();

    program.push_str("
#include <inttypes.h>
#include <iostream>
#include <stack>
#include <string>
#include <vector>

enum class ArgType
{
    bite,
    nom,
    chomp,
    drift,
    charms
};
        
union Arg
{
    const char* bite;
    const char* nom;
    const char* chomp;
    const char* drift;
    uint8_t charms;
};
        
struct ArgWrap
{
    ArgType t;
    std::vector<Arg> a;
};
        
int main()
{
    std::stack<ArgWrap> stack;
                    ");

    let mut expects: Vec<alpwltgybe::GybeTkn> = vec![alpwltgybe::GybeTkn::IDEN];
    let mut arg_counter: u64 = 0;
    let mut in_quote: bool = false;
    let mut var_counter: u64 = 0;
    let mut curr_type: &String = &String::new();
    let mut idx: usize = 0;
    while idx < tokens.len()
    {
        if expects.contains(&tokens[idx].key)
        {
            if idx != 0
            {
                expects.clear();
            }

            match tokens[idx].key
            {
                alpwltgybe::GybeTkn::NUM =>
                {
                    if curr_type.as_str() == "charms"
                    {
                        arg_counter += 1;
                        let mut temp: String = String::new();
                        write!(temp, "
    Arg arg{1};
    arg{1}.{3} = {2};
    var{0}.a.push_back(arg{1});
                        ", var_counter, arg_counter, tokens[idx].value, curr_type).unwrap();
                        program.push_str(temp.as_str());

                        expects.push(alpwltgybe::GybeTkn::PERIOD);
                        expects.push(alpwltgybe::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "sub"
                    {
                        match tokens[idx].value.parse::<i32>().unwrap_or(-1)
                        {
                            -1 => // ERROR
                            {
                                return String::new();
                            }
                            2 =>
                            {
                                let mut temp: String = String::new();
                                write!(temp, "
    if (var{0}.t == ArgType::charms)
    {{
        stack.pop();
        for (Arg a : var{0}.a)
        {{
            printf(\"%c\", a.charms);
        }}
    }}
                                ", var_counter).unwrap();
                                program.push_str(temp.as_str());
                            }
                            _ =>
                            {

                            }
                        }

                        expects.push(alpwltgybe::GybeTkn::SEMI);
                    }
                }
                alpwltgybe::GybeTkn::IDEN =>
                {
                    if tokens[idx].value.as_str() == "bite" ||
                        tokens[idx].value.as_str() == "nom" ||
                        tokens[idx].value.as_str() == "chomp" ||
                        tokens[idx].value.as_str() == "drift" ||
                        tokens[idx].value.as_str() == "charms"
                    {
                        curr_type = &tokens[idx].value;
                        var_counter += 1; 

                        let mut temp: String = String::new();
                        write!(temp, "
    ArgWrap var{0};
    var{0}.t = ArgType::{1};
                        ", var_counter, curr_type).unwrap();
                        program.push_str(temp.as_str());                          
                    }
                    else if tokens[idx].value.as_str() == "sub"
                    {
                        curr_type = &tokens[idx].value;
                    }
                    else if tokens[idx - 1].value.as_str() == "give" &&
                        tokens[idx].value.as_str() == "up"
                    {
                        let mut temp: String = String::new();
                        write!(temp, "
    return 0;
                        ").unwrap();
                        program.push_str(temp.as_str());
                    }
                    else if in_quote
                    {
                        arg_counter += 1;
                        let mut temp: String = String::new();
                        write!(temp, "
    Arg arg{1};
    arg{1}.{3} = \"{2}\";
    var{0}.a.push_back(arg{1});
                        ", var_counter, arg_counter, tokens[idx].value, curr_type).unwrap();
                        program.push_str(temp.as_str());

                        if tokens[idx + 1].key == alpwltgybe::GybeTkn::QUOTE
                        {
                            let mut temp: String = String::new();
                            write!(temp, "
    stack.push(var{});
                            ", var_counter).unwrap();
                            program.push_str(temp.as_str());
                        }
                    }

                    if !in_quote
                    {
                        expects.push(alpwltgybe::GybeTkn::IDEN);
                        expects.push(alpwltgybe::GybeTkn::SEMI);
                    }

                    if curr_type.as_str() == "charms"
                    {
                        expects.push(alpwltgybe::GybeTkn::NUM);
                    }
                    else if curr_type.as_str() == "sub"
                    {
                        expects.push(alpwltgybe::GybeTkn::NUM);
                    }
                    else if curr_type.as_str() == "up" // Give up
                    {
                        expects.clear();
                        expects.push(alpwltgybe::GybeTkn::SEMI)
                    }
                }
                alpwltgybe::GybeTkn::QUOTE =>
                {
                    if in_quote
                    {
                        expects.push(alpwltgybe::GybeTkn::PERIOD);
                        expects.push(alpwltgybe::GybeTkn::SEMI);
                        in_quote = false;   
                    }
                    else
                    {
                        expects.push(alpwltgybe::GybeTkn::IDEN);
                        expects.push(alpwltgybe::GybeTkn::NUM);
                        expects.push(alpwltgybe::GybeTkn::QUOTE);
                        in_quote = true;
                    }
                }
                alpwltgybe::GybeTkn::SEMI =>
                {
                    if (tokens[idx - 1].key == alpwltgybe::GybeTkn::NUM &&
                        tokens[idx - 2].key != alpwltgybe::GybeTkn::IDEN) ||
                        (tokens[idx - 1].key == alpwltgybe::GybeTkn::QUOTE &&
                        tokens[idx - 2].key == alpwltgybe::GybeTkn::IDEN)
                    {
                        let mut temp: String = String::new();
                        write!(temp, "
    stack.push(var{});
                        ", var_counter).unwrap();
                        program.push_str(temp.as_str());
                    }
                    expects.push(alpwltgybe::GybeTkn::IDEN);
                }
                alpwltgybe::GybeTkn::PERIOD =>
                {
                    expects.push(alpwltgybe::GybeTkn::NUM);
                    expects.push(alpwltgybe::GybeTkn::IDEN);
                }
                alpwltgybe::GybeTkn::ILLEGAL => // ERROR
                {

                }
            }
        }
        else // ERROR
        {
            return String::new();
        }
        idx += 1;
    }

    program.push_str("
}
                    ");
    return program;
}