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

use crate::parser;

#[derive(Clone)]
pub struct Arg
{
    pub elm: Vec<u8>,
    pub typ: DataTypes
}

pub struct Command
{
    pub arg: Arg,
    pub func: FuncTypes,
    pub line: u64
}

#[derive(PartialEq)]
pub enum FuncTypes
{
    BIT,
    CRM,
    CMP,
    DIF,
    DUP,
    END,
    PED,
    LOP,
    PRT,
    PCT,
    QUT,
    RED,
    REM,
    SKP,
    SUB,
    SUM,
    SWP
}

#[derive(Clone, PartialEq)]
pub enum DataTypes
{
    BITE,
    CHRM,
    NONE
}

pub fn interpret(commands: parser::Parsed)
{
    let mut gybe_stack: std::collections::VecDeque<Arg> = std::collections::VecDeque::new();
    let mut loop_stack: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    let mut loop_indicies: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    let mut loop_depth: usize = 0;

    let mut end: bool = false;

    let mut lin_idx: usize = 1;
    let mut cmd_idx: usize = 0;
    while lin_idx < commands.lines as usize
    {
        if end
        {
            break;
        }

        if commands.call[cmd_idx].line as usize == lin_idx
        {
            match &commands.call[cmd_idx].func
            {
                FuncTypes::BIT | FuncTypes::CRM =>
                {
                    gybe_stack.push_front(commands.call[cmd_idx].arg.clone());  
                }
                FuncTypes::CMP =>
                {
                    let lhs: Arg = gybe_stack.pop_front().unwrap();
                    let rhs: Arg = gybe_stack.pop_front().unwrap();

                    if lhs.typ != rhs.typ
                    {
                        println!("RUNTIME-ERROR: Tried to compare 2 elements of different types");
                        end = true;
                    }

                    if lhs.elm[0] == rhs.elm[0]
                    {
                        gybe_stack.push_front(Arg{elm: vec![1], typ: DataTypes::BITE})
                    }
                    else
                    {
                        gybe_stack.push_front(Arg{elm: vec![0], typ: DataTypes::BITE})
                    }
                }
                FuncTypes::DIF | FuncTypes::PCT | FuncTypes::QUT | FuncTypes::REM | FuncTypes::SUM =>
                {
                    let lhs: Arg = gybe_stack.pop_front().unwrap();
                    let rhs: Arg = gybe_stack.pop_front().unwrap();

                    if lhs.typ != rhs.typ
                    {
                        println!("RUNTIME-ERROR: Tried to do an operation on 2 elements of different types");
                        end = true;
                    }

                    let len: usize;
                    if lhs.elm.len() > rhs.elm.len()
                    {
                        len = rhs.elm.len();
                    }
                    else
                    {
                        len = lhs.elm.len();
                    }

                    let mut n_arg: Arg = Arg{elm: Vec::new(), typ: lhs.typ};

                    match &commands.call[cmd_idx].func
                    {
                        FuncTypes::DIF =>
                        {
                            for i in 0..len
                            {
                                n_arg.elm.push(lhs.elm[i] - rhs.elm[i]);
                            }
                        }
                        FuncTypes::PCT =>
                        {
                            for i in 0..len
                            {
                                n_arg.elm.push(lhs.elm[i] * rhs.elm[i]);
                            }
                        }
                        FuncTypes::QUT =>
                        {
                            for i in 0..len
                            {
                                n_arg.elm.push(lhs.elm[i] / rhs.elm[i]);
                            }
                        }
                        FuncTypes::REM =>
                        {
                            for i in 0..len
                            {
                                n_arg.elm.push(lhs.elm[i] % rhs.elm[i]);
                            }
                        }
                        FuncTypes::SUM =>
                        {
                            for i in 0..len
                            {
                                n_arg.elm.push(lhs.elm[i] + rhs.elm[i]);
                            }
                        }
                        _ => // Should be impossible to get here
                        {
                            println!("RUNTIME-ERROR: ???");
                            end = true;
                        }
                    }

                    gybe_stack.push_front(n_arg);
                }
                FuncTypes::DUP =>
                {
                    gybe_stack.push_front(Arg{elm: gybe_stack.front().unwrap().elm.clone(), typ: gybe_stack.front().unwrap().typ.clone()});
                }
                FuncTypes::END =>
                {
                    if gybe_stack.front().unwrap().elm.clone()[0] != 0 // Loop
                    {
                        cmd_idx = loop_indicies[loop_depth - 1];
                        lin_idx = loop_stack[loop_depth - 1];
                        continue;
                    }
                    else // Break
                    {
                        loop_depth -= 1;
                        loop_indicies.pop_back();
                        loop_stack.pop_back();
                    }
                }
                FuncTypes::PED =>
                {
                    end = true;
                }
                FuncTypes::LOP =>
                {
                    loop_depth += 1;
                    loop_indicies.push_front(cmd_idx);
                    loop_stack.push_front(lin_idx);

                    if gybe_stack.front().unwrap().elm.clone()[0] == 0 // Skip loop if condition is not met
                    {
                        let mut i: usize = loop_depth;
                        while i != 0
                        {
                            cmd_idx += 1;
                            if commands.call[cmd_idx].func == FuncTypes::END
                            {
                                i -= 1;
                            }
                        }
                        lin_idx = commands.call[cmd_idx].line as usize;
                    }
                }
                FuncTypes::PRT =>
                {
                    match &gybe_stack.front().unwrap().typ
                    {
                        DataTypes::BITE =>
                        {
                            for i in &gybe_stack.front().unwrap().elm
                            {
                                print!("{}", i)
                            }
                        }
                        DataTypes::CHRM =>
                        {
                            for i in &gybe_stack.front().unwrap().elm
                            {
                                print!("{}", *i as char)
                            }
                        }
                        DataTypes::NONE =>
                        {
                            println!("RUNTIME-ERROR: Tried to print non-exsistant data type");
                            end = true;
                        }
                    }

                    gybe_stack.pop_front();
                }
                FuncTypes::RED =>
                {
                    let mut line: String = String::new();
                    std::io::stdin().read_line(&mut line).unwrap();
                    let chr: Vec<char> = line.chars().collect();
                    let mut arg: Vec<u8> = Vec::new();
                    for c in chr
                    {
                        arg.push(c as u8);
                    }
                    gybe_stack.push_front(Arg{elm: arg, typ: DataTypes::CHRM})
                }
                FuncTypes::SKP =>
                {
                    let a: Arg = gybe_stack.pop_front().unwrap();

                    if a.elm[0] as u8 != 0
                    {
                        lin_idx += commands.call[cmd_idx].arg.elm[0] as usize;
                    }

                    // Snap to the closest command to a given line number, rounded down
                    while commands.call[cmd_idx].line as usize != lin_idx && 
                    cmd_idx + 1 < commands.call.len() && 
                    commands.call[cmd_idx + 1].line as usize  <= lin_idx
                    {
                        cmd_idx += 1;
                    }
                }
                FuncTypes::SUB =>
                {
                    
                }
                FuncTypes::SWP =>
                {
                    gybe_stack.swap(0, 1);
                }
            }
            cmd_idx += 1;
        }
        else
        {
            if cmd_idx < commands.call.len() &&
            commands.call[cmd_idx].line as usize != lin_idx
            {
                lin_idx += 1;
            }
        }
    }
}