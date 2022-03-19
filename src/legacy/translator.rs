use crate::lexer;
use crate::c_code;

use std::collections::VecDeque;

// This is really messy if someone wants to clean it up go ahead but that's not gonna be my job

// I understand that this is a pretty miserable way of code parsing, if you can even call it that.
// The code here was written to be a fast hack-y implementation and not to be expandable, reuseable, or fast.

pub struct Label
{
    idx: u64,
    lin: u64
}

pub fn translate(tokens: Vec<lexer::Token>) -> String
{
    let mut program: String = String::new();
    program.push_str(c_code::get_begin_string().as_str());

    let mut expects: Vec<lexer::GybeTkn> = vec![lexer::GybeTkn::IDEN];
    let mut types: VecDeque<&String> = VecDeque::new();
    let mut subs: Vec<i32> = Vec::new();
    let mut labels: Vec<Label> = Vec::new();

    let mut lbl_counter: u64 = 0; // Used for label naming
    let mut var_counter: u64 = 0; // Used for variable naming

    let mut in_quote: bool = false;

    let charms_string: String = String::from("charms"); // I don't like having to do this but w/e

    let mut idx: usize = 0;
    while idx < tokens.len()
    {
        let prev: &lexer::Token;
        let curr: &lexer::Token = &tokens[idx];

        let curr_type: &String;

        if types.len() > 0
        {
            curr_type = &types[0];
        }
        else
        {
            types.push_front(&curr.value);
            curr_type = &types[0];
        }


        if idx > 0
        {
            prev = &tokens[idx - 1];
        }
        else
        {
            prev = &tokens[idx];
        }

        expects.push(lexer::GybeTkn::NEWLIN);
        expects.push(lexer::GybeTkn::ILLEGAL);
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
                    else if prev.value.as_str() == "sub" // NOT FINISHED
                    {
                        let sub_key = curr.value.parse::<i32>().unwrap_or(-1);

                        if subs.contains(&sub_key)
                        {

                        }
                        else
                        {
                            subs.push(sub_key);
                        }

                        expects.push(lexer::GybeTkn::SEMI);
                    }
                }
                lexer::GybeTkn::IDEN =>
                {
                    if in_quote
                    {
                        if tokens.len() > 2 && tokens[idx - 2].value.as_str() == "skip"
                        {
                            lbl_counter += 1;
                            labels.push(Label { idx: lbl_counter, lin: curr.value.chars().nth(0).unwrap() as u64 });
                            program.push_str(c_code::get_skip_string(lbl_counter).as_str());
                        } 
                        else
                        {
                            program.push_str(c_code::get_arg_string(var_counter, &curr.value, curr_type).as_str());
                        }
                        expects.push(lexer::GybeTkn::QUOTE);
                    }
                    else if curr.value.as_str() == "bite" ||
                        curr.value.as_str() == "charms"
                    {
                        var_counter += 1; 
                        types.push_front(&curr.value);
                        program.push_str(c_code::get_arg_wrap_string(var_counter).as_str());  

                        if curr.value.as_str() == "charms"
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
                    else if curr.value.as_str() == "comp"
                    {
                        types.push_front(curr_type); // Reverse order here
                        types.pop_front();
                        types.pop_front();

                        var_counter += 1; 
                        program.push_str(c_code::get_comp_string(var_counter).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "difference"
                    {
                        types.push_front(curr_type); // Reverse order here
                        types.pop_front();
                        types.pop_front();

                        var_counter += 1; 
                        program.push_str(c_code::get_difference_string(var_counter).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "dup"
                    {
                        types.push_front(curr_type);

                        program.push_str(c_code::get_dup_string().as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "end"
                    {
                        program.push_str(c_code::get_end_string().as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "give"
                    {
                        expects.push(lexer::GybeTkn::IDEN);
                    }
                    else if curr.value.as_str() == "loop"
                    {
                        //if curr_type != "charms" // Declaration type
                        //{
                            program.push_str(c_code::get_while_string().as_str());
                        //}
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "print"
                    {
                        types.pop_front();

                        program.push_str(c_code::get_print_string(curr_type).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "product"
                    {
                        types.push_front(curr_type); // Reverse order here
                        types.pop_front();
                        types.pop_front();

                        var_counter += 1; 
                        program.push_str(c_code::get_product_string(var_counter).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "quotient"
                    {
                        types.push_front(curr_type); // Reverse order here
                        types.pop_front();
                        types.pop_front();

                        var_counter += 1; 
                        program.push_str(c_code::get_quotient_string(var_counter).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "read"
                    {
                        types.push_front(&charms_string);

                        var_counter += 1;
                        program.push_str(c_code::get_read_string(var_counter).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "remainder"
                    {
                        types.push_front(curr_type); // Reverse order here
                        types.pop_front();
                        types.pop_front();

                        var_counter += 1; 
                        program.push_str(c_code::get_remainder_string(var_counter).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "skip"
                    {
                        expects.push(lexer::GybeTkn::QUOTE);
                    }
                    else if curr.value.as_str() == "sub"
                    {
                        expects.push(lexer::GybeTkn::NUM);
                    }
                    else if curr.value.as_str() == "sum"
                    {
                        types.push_front(curr_type); // Reverse order here
                        types.pop_front();
                        types.pop_front();

                        var_counter += 1; 
                        program.push_str(c_code::get_sum_string(var_counter).as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "swap"
                    {
                        let a: &String = curr_type;
                        types.pop_front();
                        let b: &String = curr_type;
                        types.pop_front();
                        types.push_front(a);
                        types.push_front(b);

                        program.push_str(c_code::get_swap_string().as_str());
                        expects.push(lexer::GybeTkn::SEMI);
                    }
                    else if curr.value.as_str() == "up"
                    {
                        program.push_str(c_code::get_return_string().as_str());
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
                    if (prev.key == lexer::GybeTkn::NUM ||
                        (prev.key == lexer::GybeTkn::QUOTE &&
                        tokens[idx - 2].key == lexer::GybeTkn::IDEN)) &&
                        tokens[idx - 4].value.as_str() != "skip"
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
                lexer::GybeTkn::NEWLIN =>
                {
                    if labels.len() > 0
                    {
                        let mut lbl_idx: usize = 0; 
                        while lbl_idx < labels.len() // While loop is used to not break loop when removing elements
                        {
                            if labels[lbl_idx].lin > 0
                            {
                                labels[lbl_idx].lin -= 1;
                            }
                            else
                            {
                                program.push_str(c_code::get_label_string(labels[lbl_idx].idx as u64).as_str());
                                labels.remove(lbl_idx);
                                continue;
                            }
                            lbl_idx += 1;
                        } 
                    }
                    expects.push(lexer::GybeTkn::NUM);
                    expects.push(lexer::GybeTkn::IDEN);
                }
                lexer::GybeTkn::ILLEGAL => // ERROR
                {
                    println!("ILLEGAL ACTION: {}", &curr.value);
                    return String::new();
                }
            }
        }
        else // ERROR
        {
            for tkn in &expects
            {
                println!("COMPILATION FAILED: expected {:?}, found {:?}", tkn , &curr.key);
                return String::new();
            }
        }
        idx += 1;
    }

    return program;
}