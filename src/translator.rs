use crate::lexer;
use crate::c_code;

// This is really messy if someone wants to clean it up go ahead but that's not gonna be my job

pub fn translate(tokens: Vec<lexer::Token>) -> String
{
    let mut program: String = String::new();
    program.push_str(c_code::get_begin_string().as_str());

    let mut expects: Vec<lexer::GybeTkn> = vec![lexer::GybeTkn::IDEN];

    let mut var_counter: u64 = 0;                   // Used for variable naming
    let mut in_quote: bool = false;
    let mut curr_type: &String = &String::new();    // Current type the translator is working with
    let mut top_idx: usize = 0;                     // Index to the token at the top of the C++ stack

    let mut idx: usize = 0;
    while idx < tokens.len()
    {
        let prev: &lexer::Token;
        let curr: &lexer::Token = &tokens[idx];

        if idx > 0
        {
            prev = &tokens[idx - 1];
        }
        else
        {
            prev = &tokens[idx];
        }

        if expects.contains(&curr.key)
        {
            if idx != 0
            {
                expects.clear();
            }

            match curr.key
            {
                lexer::GybeTkn::NUM =>
                {
                    if curr_type.as_str() == "charms"
                    {
                        program.push_str(c_code::get_arg_string(var_counter, &curr.value, curr_type).as_str());

                        expects.push(lexer::GybeTkn::PERIOD);
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "sub"
                    {
                        match curr.value.parse::<i32>().unwrap_or(-1)
                        {
                            -1 => // ERROR
                            {
                                return String::new();
                            }
                            _ =>
                            {

                            }
                        }

                        expects.push(lexer::GybeTkn::SEMI);
                    }
                }
                lexer::GybeTkn::IDEN =>
                {
                    if !in_quote
                    {
                        curr_type = &curr.value;
                    }

                    if in_quote
                    {
                        program.push_str(c_code::get_arg_string(var_counter, &curr.value, curr_type).as_str());
                        expects.push(lexer::GybeTkn::QUOTE);
                    }
                    else if curr_type.as_str() == "bite" ||
                        curr_type.as_str() == "nom" ||
                        curr_type.as_str() == "chomp" ||
                        curr_type.as_str() == "drift" ||
                        curr_type.as_str() == "charms"
                    {
                        var_counter += 1; 
                        top_idx = idx;
                        program.push_str(c_code::get_arg_wrap_string(var_counter, curr_type).as_str());  
                        
                        if curr_type.as_str() == "charms"
                        {
                            expects.push(lexer::GybeTkn::NUM);
                        }
                        else
                        {
                            expects.push(lexer::GybeTkn::QUOTE);
                            expects.push(lexer::GybeTkn::IDEN);
                            expects.push(lexer::GybeTkn::SEMI);
                        }
                    }
                    else if curr_type.as_str() == "dup"
                    {
                        program.push_str(c_code::get_dup_string().as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "print"
                    {
                        program.push_str(c_code::get_print_string(&tokens[top_idx].value).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "sub"
                    {
                        expects.push(lexer::GybeTkn::NUM);
                    }
                    else if curr_type.as_str() == "give"
                    {
                        expects.push(lexer::GybeTkn::IDEN);
                    }
                    else if curr_type.as_str() == "up"
                    {
                        program.push_str(c_code::get_return_string().as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "end"
                    {
                        program.push_str(c_code::get_end_string().as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "loop"
                    {
                        if tokens[top_idx].value != "charms" // Declaration type
                        {
                            program.push_str(c_code::get_while_string(&tokens[top_idx].value).as_str())
                        }
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "sum"
                    {
                        var_counter += 1; 
                        program.push_str(c_code::get_sum_string(var_counter, &tokens[top_idx].value).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "difference"
                    {
                        var_counter += 1; 
                        program.push_str(c_code::get_difference_string(var_counter, &tokens[top_idx].value).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "product"
                    {
                        var_counter += 1; 
                        program.push_str(c_code::get_product_string(var_counter, &tokens[top_idx].value).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "quotient"
                    {
                        var_counter += 1; 
                        program.push_str(c_code::get_quotient_string(var_counter, &tokens[top_idx].value).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "remainder"
                    {
                        var_counter += 1; 
                        program.push_str(c_code::get_remainder_string(var_counter, &tokens[top_idx].value).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr_type.as_str() == "swap"
                    {
                        program.push_str(c_code::get_swap_string().as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                }
                lexer::GybeTkn::QUOTE =>
                {
                    if in_quote
                    {
                        expects.push(lexer::GybeTkn::PERIOD);
                        expects.push(lexer::GybeTkn::SEMI);
                        in_quote = false;   
                    }
                    else
                    {
                        expects.push(lexer::GybeTkn::IDEN);
                        expects.push(lexer::GybeTkn::NUM);
                        expects.push(lexer::GybeTkn::QUOTE);
                        in_quote = true;
                    }
                }
                lexer::GybeTkn::SEMI =>
                {
                    if (prev.key == lexer::GybeTkn::NUM &&
                        tokens[idx - 2].key != lexer::GybeTkn::IDEN) ||
                        (prev.key == lexer::GybeTkn::QUOTE &&
                        tokens[idx - 2].key == lexer::GybeTkn::IDEN)
                    {
                        program.push_str(c_code::get_push_string(var_counter).as_str());
                    }
                    expects.push(lexer::GybeTkn::IDEN);
                }
                lexer::GybeTkn::PERIOD =>
                {
                    expects.push(lexer::GybeTkn::NUM);
                    expects.push(lexer::GybeTkn::IDEN);
                }
                lexer::GybeTkn::ILLEGAL => // ERROR
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

    return program;
}