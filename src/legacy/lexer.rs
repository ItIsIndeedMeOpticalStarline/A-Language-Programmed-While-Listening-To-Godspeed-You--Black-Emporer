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

#[derive(PartialEq)]
#[derive(Debug)]
pub enum GybeTkn
{
    NUM,
    IDEN,

    QUOTE,
    SEMI,
    PERIOD,

    NEWLIN,
    ILLEGAL
}

pub struct Token
{
    pub key: GybeTkn,
    pub value: String,
}

// Unused. Debugging method
pub fn _tkn_print(t: Token)
{
    let s: &str;

    match t.key
    {
        GybeTkn::NUM => s = "NUM",
        GybeTkn::IDEN => s = "IDEN",
        GybeTkn::QUOTE => s = "QUOTE",
        GybeTkn::SEMI => s = "SEMI",
        GybeTkn::PERIOD => s = "PERIOD",
        GybeTkn::NEWLIN => s = "NEWLIN",
        GybeTkn::ILLEGAL => s = "ILLEGAL"
    }

    println!("{} | {}", s, t.value);
}

// This lexer is very simple, it dosen't have any logic to tokens and values and is just split by whitespace
pub fn lex(contents: String) -> Vec<Token>
{
    let mut tokens: Vec<Token> = Vec::new();

    let chars: Vec<char> = contents.chars().collect();

    let mut idx: usize = 0;
    while idx < chars.len()
    {  
        let temp_key: GybeTkn;
        let mut temp_value: String = String::new();

        let next: char = chars[idx];

        if next.is_ascii_whitespace() || next.is_ascii_control()
        {
            if next == '\n'
            {
                tokens.push(Token{key: GybeTkn::NEWLIN, value: temp_value});
            }

            idx += 1;
            continue;
        }

        if next.is_ascii_alphabetic()
        {
            while chars[idx].is_ascii_alphabetic()
            {
                temp_value.push(chars[idx]);
                idx += 1;
            }

            tokens.push(Token{key: GybeTkn::IDEN, value: temp_value});
            continue;
        }

        if next.is_ascii_digit()
        {
            while chars[idx].is_ascii_digit()
            {
                temp_value.push(chars[idx]);             
                idx += 1;
            }

            tokens.push(Token{key: GybeTkn::NUM, value: temp_value});
            continue;
        }

        match next
        {
            '\'' =>
            {
                temp_key = GybeTkn::QUOTE;
            }
            '﹔' =>
            {
                temp_key = GybeTkn::SEMI;
            }
            '.' =>
            {
                temp_key = GybeTkn::PERIOD;
            }
            _ =>
            {
                temp_key = GybeTkn::ILLEGAL;
            }
        }      

        tokens.push(Token{ key: temp_key, value: temp_value });
        idx += 1;
    }

    return tokens;
}