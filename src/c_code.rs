use std::fmt::Write;

pub fn get_arg_string(num_vars: u64, arg_value: &String, arg_type: &String) -> String
{
    let mut arg_string: String = String::new();

    if arg_type.as_str() == "charms"
    {
        write!(arg_string, "
        var{0}.a.push_back(Arg(ArgType::{1}, \' \', {2}));
        ", num_vars, arg_type, arg_value).expect("get_arg_string(u64, &String, &String) failed");
    }
    else
    {
        write!(arg_string, "
        var{0}.a.push_back(Arg(ArgType::{1}, \'{2}\', 0));
        ", num_vars, arg_type, arg_value).expect("get_arg_string(u64, &String, &String) failed");
    }

    return arg_string;
}

pub fn get_arg_wrap_string(num_vars: u64, arg_type: &String) -> String
{
    let mut arg_wrap_string: String = String::new();

    write!(arg_wrap_string, "
    ArgWrap var{0}(ArgType::{1});
    ", num_vars, arg_type).expect("get_arg_wrap_string(u64, &String) failed");

    return arg_wrap_string;
}

pub fn get_begin_string() -> String
{
    let begin_string: String = String::from("
    #include <cassert>
    #include <inttypes.h>
    #include <iostream>
    #include <stack>
    #include <string>
    #include <vector>

    enum class ArgType
    {
        none,
        bite,
        nom,
        chomp,
        drift,
        charms
    };

    union Arg
    {
        char bite;
        char nom;
        char chomp;
        char drift;
        uint8_t charms;

        Arg(ArgType t, char dS = NULL, uint8_t dI = 0)
        {
            switch (t)
            {
                case ArgType::bite:
                    bite = dS;
                    break;
                case ArgType::nom:
                    nom = dS;
                    break;
                case ArgType::chomp:
                    chomp = dS;
                    break;
                case ArgType::drift:
                    drift = dS;
                    break;
                case ArgType::charms:
                    charms = dI;
                    break;
                default:
                    printf(\"FATAL ERROR: WHAT THE FUCK\");
                    break;
            }
        }
    };

    struct ArgWrap
    {
        ArgType t;
        std::vector<Arg> a;

        ArgWrap(ArgType dT)
            : t(dT)
        {   }

        ArgWrap()
            : t(ArgType::none)
        {   }
    };

    ArgWrap regA;
    ArgWrap regB;

    int main()
    {
        std::stack<ArgWrap> stack;
    ");

    return begin_string;
}

pub fn get_difference_string(num_vars: u64, arg_type: &String) -> String
{
    let mut difference_string: String = String::new();

    write!(difference_string, "
    assert(stack.top().t != ArgType::charms);
    ArgWrap var{0}(ArgType::{1});
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    if (regA.a.size() > regB.a.size())
    {{
        for (size_t i = 0; i < regB.a.size(); i++)
        {{
            var{0}.a.push_back(Arg(ArgType::{1}, (char)((int)regA.a.at(i).{1} - (int)regB.a.at(i).{1}), 0));
        }}
    }}
    else
    {{
        for (size_t i = 0; i < regA.a.size(); i++)
        {{
            var{0}.a.push_back(Arg(ArgType::{1}, (char)((int)regA.a.at(i).{1} - (int)regB.a.at(i).{1}), 0));
        }}
    }}
    stack.push(var{0});
    ", num_vars, arg_type).expect("get_difference_string(u64, &String) failed");

    return difference_string;
}

pub fn get_dup_string() -> String
{
    let dup_string: String = String::from("
    stack.push(stack.top());
    ");

    return dup_string;
}

pub fn get_end_string() -> String
{
    let end_string: String = String::from("
    }
    ");

    return end_string;
}

pub fn get_push_string(num_vars: u64) -> String
{
    let mut push_string: String = String::new();

    write!(push_string, "
    stack.push(var{});
    ", num_vars).expect("get_push_string(u64) failed");

    return push_string;
}

pub fn get_print_string(arg_type: &String) -> String
{
    let mut print_string: String = String::new();
    
    if arg_type.as_str() == "charms"
    {
        write!(print_string, "
        for (Arg a : stack.top().a)
        {{
            printf(\"%c\", a.{});
        }}
        stack.pop();
        ", arg_type).expect("get_print_string(&String) failed");
    }
    else
    {
        write!(print_string, "
        for (Arg a : stack.top().a)
        {{
            printf(\"%i\", a.{});
        }}
        stack.pop();
        ", arg_type).expect("get_print_string(&String) failed");
    }

    return print_string;
}

pub fn get_product_string(num_vars: u64, arg_type: &String) -> String
{
    let mut product_string: String = String::new();

    write!(product_string, "
    assert(stack.top().t != ArgType::charms);
    ArgWrap var{0}(ArgType::{1});
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    if (regA.a.size() > regB.a.size())
    {{
        for (size_t i = 0; i < regB.a.size(); i++)
        {{
            var{0}.a.push_back(Arg(ArgType::{1}, (char)((int)regA.a.at(i).{1} * (int)regB.a.at(i).{1}), 0));
        }}
    }}
    else
    {{
        for (size_t i = 0; i < regA.a.size(); i++)
        {{
            var{0}.a.push_back(Arg(ArgType::{1}, (char)((int)regA.a.at(i).{1} * (int)regB.a.at(i).{1}), 0));
        }}
    }}
    stack.push(var{0});
    ", num_vars, arg_type).expect("get_product_string(u64, &String) failed");

    return product_string;
}

pub fn get_quotient_string(num_vars: u64, arg_type: &String) -> String
{
    let mut quotient_string: String = String::new();

    write!(quotient_string, "
    assert(stack.top().t != ArgType::charms);
    ArgWrap var{0}(ArgType::{1});
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    if (regA.a.size() > regB.a.size())
    {{
        for (size_t i = 0; i < regB.a.size(); i++)
        {{
            var{0}.a.push_back(Arg(ArgType::{1}, (char)((int)regA.a.at(i).{1} / (int)regB.a.at(i).{1}), 0));
        }}
    }}
    else
    {{
        for (size_t i = 0; i < regA.a.size(); i++)
        {{
            var{0}.a.push_back(Arg(ArgType::{1}, (char)((int)regA.a.at(i).{1} / (int)regB.a.at(i).{1}), 0));
        }}
    }}
    stack.push(var{0});
    ", num_vars, arg_type).expect("get_quotient_string(u64, &String) failed");

    return quotient_string;
}

pub fn get_remainder_string(num_vars: u64, arg_type: &String) -> String
{
    let mut remainder_string: String = String::new();

    write!(remainder_string, "
    assert(stack.top().t != ArgType::charms);
    ArgWrap var{0}(ArgType::{1});
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    if (regA.a.size() > regB.a.size())
    {{
        for (size_t i = 0; i < regB.a.size(); i++)
        {{
            var{0}.a.push_back(Arg(ArgType::{1}, (char)((int)regA.a.at(i).{1} % (int)regB.a.at(i).{1}), 0));
        }}
    }}
    else
    {{
        for (size_t i = 0; i < regA.a.size(); i++)
        {{
            var{0}.a.push_back(Arg(ArgType::{1}, (char)((int)regA.a.at(i).{1} % (int)regB.a.at(i).{1}), 0));
        }}
    }}
    stack.push(var{0});
    ", num_vars, arg_type).expect("get_remainder_string(u64, &String) failed");

    return remainder_string;
}

pub fn get_return_string() -> String
{
    let return_string: String = String::from("
        return 0;
    }
    ");

    return return_string;
}

pub fn get_sum_string(num_vars: u64, arg_type: &String) -> String
{
    let mut sum_string: String = String::new();

    write!(sum_string, "
    assert(stack.top().t != ArgType::charms);
    ArgWrap var{0}(ArgType::{1});
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    if (regA.a.size() > regB.a.size())
    {{
        for (size_t i = 0; i < regB.a.size(); i++)
        {{
            var{0}.a.push_back(Arg(ArgType::{1}, (char)((int)regA.a.at(i).{1} + (int)regB.a.at(i).{1}), 0));
        }}
    }}
    else
    {{
        for (size_t i = 0; i < regA.a.size(); i++)
        {{
            var{0}.a.push_back(Arg(ArgType::{1}, (char)((int)regA.a.at(i).{1} + (int)regB.a.at(i).{1}), 0));
        }}
    }}
    stack.push(var{0});
    ", num_vars, arg_type).expect("get_sum_string(u64, &String) failed");

    return sum_string;
}

pub fn get_swap_string() -> String
{
    let swap_string: String = String::from("
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    stack.push(regA);
    stack.push(regB);
    ");

    return swap_string;
}

pub fn get_while_string(arg_type: &String) -> String
{
    let mut while_string: String = String::new();
    
    write!(while_string, "
    assert(stack.top().t != ArgType::charms);
    assert(stack.top().a.size() == 1);
    while ((int)stack.top().a.at(0).{0} != 0)
    {{
        //stack.top().a.at(0).{0} = (char)((int)stack.top().a.at(0).{0} - 1);
    ", arg_type).expect("get_while_string(&String) failed");

    return while_string;
}