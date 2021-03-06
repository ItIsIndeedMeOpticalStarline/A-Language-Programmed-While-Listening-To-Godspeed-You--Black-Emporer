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

use std::fmt::Write;

// !!! NEVER DO THIS FOR YOUR LANGUAGE, IT IS MUCH HARDER TO WRITE AND USE. MAKE AN INTERPRETER INSTEAD !!!

pub fn get_arg_string(num_vars: u64, arg_value: &String, arg_type: &String) -> String
{
    let mut arg_string: String = String::new();

    if arg_type.as_str() == "charms"
    {
        write!(arg_string, "
        var{0}.a.push_back({1});
        ", num_vars, arg_value).expect("get_arg_string(u64, &String, &String) failed");
    }
    else
    {
        write!(arg_string, "
        var{0}.a.push_back(\'{1}\');
        ", num_vars, arg_value).expect("get_arg_string(u64, &String, &String) failed");
    }

    return arg_string;
}

pub fn get_arg_wrap_string(num_vars: u64) -> String
{
    let mut arg_wrap_string: String = String::new();

    write!(arg_wrap_string, "
    ArgWrap var{0};
    ", num_vars).expect("get_arg_wrap_string(u64) failed");

    return arg_wrap_string;
}

pub fn get_begin_string() -> String
{
    let begin_string: String = String::from("
    #include <inttypes.h>
    #include <iostream>
    #include <stack>
    #include <string>
    #include <vector>


    struct ArgWrap
    {
        std::vector<int8_t> a;

        ArgWrap()
            : a(NULL)
        {   }

        ArgWrap(int8_t dA)
            : a(dA)
        {   }
    };

    ArgWrap regA;
    ArgWrap regB;

    std::string ioReg;

    int main()
    {
        std::stack<ArgWrap> stack;
    ");

    return begin_string;
}

pub fn get_comp_string(num_vars: u64) -> String
{
    let mut comp_string: String = String::new();

    write!(comp_string, "
    ArgWrap var{0};
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    var{0}.a.push_back((regA.a.at(0) == regB.a.at(0)) ? 1 : 0);
    stack.push(var{0});
    ", num_vars).expect("get_comp_string(u64) failed");

    return comp_string;
}

pub fn get_difference_string(num_vars: u64) -> String
{
    let mut difference_string: String = String::new();

    write!(difference_string, "
    ArgWrap var{0};
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    if (regA.a.size() > regB.a.size())
    {{
        for (size_t i = 0; i < regB.a.size(); i++)
        {{
            var{0}.a.push_back(regA.a.at(i) - regB.a.at(i));
        }}
    }}
    else
    {{
        for (size_t i = 0; i < regA.a.size(); i++)
        {{
            var{0}.a.push_back(regA.a.at(i) - regB.a.at(i));
        }}
    }}
    stack.push(var{0});
    ", num_vars).expect("get_difference_string(u64) failed");

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

pub fn get_label_string(num_labels: u64) -> String
{
    let mut label_string: String = String::new();

    write!(label_string, "
    }}
    label{}:
    ", num_labels).expect("get_label_string(u64) failed");

    return label_string;
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
        for (int8_t a : stack.top().a)
        {{
            printf(\"%c\", a);
        }}
        stack.pop();
        ").expect("get_print_string(&String) failed");
    }
    else
    {
        write!(print_string, "
        for (int8_t a : stack.top().a)
        {{
            printf(\"%i\", a);
        }}
        stack.pop();
        ").expect("get_print_string(&String) failed");
    }

    return print_string;
}

pub fn get_product_string(num_vars: u64) -> String
{
    let mut product_string: String = String::new();

    write!(product_string, "
    ArgWrap var{0};
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    if (regA.a.size() > regB.a.size())
    {{
        for (size_t i = 0; i < regB.a.size(); i++)
        {{
            var{0}.a.push_back(regA.a.at(i) * regB.a.at(i));
        }}
    }}
    else
    {{
        for (size_t i = 0; i < regA.a.size(); i++)
        {{
            var{0}.a.push_back(regA.a.at(i) * regB.a.at(i));
        }}
    }}
    stack.push(var{0});
    ", num_vars).expect("get_product_string(u64) failed");

    return product_string;
}

pub fn get_quotient_string(num_vars: u64) -> String
{
    let mut quotient_string: String = String::new();

    write!(quotient_string, "
    ArgWrap var{0};
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    if (regA.a.size() > regB.a.size())
    {{
        for (size_t i = 0; i < regB.a.size(); i++)
        {{
            var{0}.a.push_back(regA.a.at(i) / regB.a.at(i));
        }}
    }}
    else
    {{
        for (size_t i = 0; i < regA.a.size(); i++)
        {{
            var{0}.a.push_back(regA.a.at(i) / regB.a.at(i));
        }}
    }}
    stack.push(var{0});
    ", num_vars).expect("get_quotient_string(u64) failed");

    return quotient_string;
}

pub fn get_read_string(num_vars: u64) -> String
{
    let mut read_string: String = String::new();

    write!(read_string, "
    ArgWrap var{0};
    std::getline(std::cin, ioReg);
    for (size_t i = 0; i < ioReg.size(); i++)
    {{
        var{0}.a.push_back((int8_t)ioReg[i]);
    }}
    stack.push(var{0});
    ", num_vars).expect("get_read_string(u64) failed");

    return read_string;
}

pub fn get_remainder_string(num_vars: u64) -> String
{
    let mut remainder_string: String = String::new();

    write!(remainder_string, "
    ArgWrap var{0};
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    if (regA.a.size() > regB.a.size())
    {{
        for (size_t i = 0; i < regB.a.size(); i++)
        {{
            var{0}.a.push_back(regA.a.at(i) % regB.a.at(i));
        }}
    }}
    else
    {{
        for (size_t i = 0; i < regA.a.size(); i++)
        {{
            var{0}.a.push_back(regA.a.at(i) % regB.a.at(i));
        }}
    }}
    stack.push(var{0});
    ", num_vars).expect("get_remainder_string(u64) failed");

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

pub fn get_skip_string(lbl: u64, ) -> String
{
    let mut skip_string: String = String::new();

    write!(skip_string, "
    if (stack.top().a.at(0) != 0)
    {{
        stack.pop();
        goto label{};
    }}
    else
    {{
        stack.pop();
    }}
    {{
    ", lbl).expect("get_skip_string(u64) failed");

    return skip_string;
}

pub fn get_sum_string(num_vars: u64) -> String
{
    let mut sum_string: String = String::new();

    write!(sum_string, "
    ArgWrap var{0};
    regA = stack.top();
    stack.pop();
    regB = stack.top();
    stack.pop();
    if (regA.a.size() > regB.a.size())
    {{
        for (size_t i = 0; i < regB.a.size(); i++)
        {{
            var{0}.a.push_back(regA.a.at(i) + regB.a.at(i));
        }}
    }}
    else
    {{
        for (size_t i = 0; i < regA.a.size(); i++)
        {{
            var{0}.a.push_back(regA.a.at(i) + regB.a.at(i));
        }}
    }}
    stack.push(var{0});
    ", num_vars).expect("get_sum_string(u64) failed");

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

pub fn get_while_string() -> String
{
    let mut while_string: String = String::new();
    
    write!(while_string, "
    while (stack.top().a.at(0) != 0)
    {{
    ").expect("get_while_string() failed");

    return while_string;
}