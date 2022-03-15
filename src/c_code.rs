use std::fmt::Write;

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
    #include <cassert>
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
    assert(stack.top().a.size() == 1);
    while (stack.top().a.at(0) != 0)
    {{
    ").expect("get_while_string() failed");

    return while_string;
}