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
pub enum GybeTkn
{
    ILLEGL,
    KEYWRD,
    LTERAL,
    SPRATR
}

pub struct Token
{
    pub key: GybeTkn,
    pub value: String,
}

pub fn lex(contents: String) -> Vec<Token>
{
    let keywords: [String; 18] =
    [
        String::from("bite"),
        String::from("charms"),
        String::from("comp"),
        String::from("difference"),
        String::from("dup"),
        String::from("end"),
        String::from("give"),
        String::from("loop"),
        String::from("print"),
        String::from("product"),
        String::from("quotient"),
        String::from("read"),
        String::from("remainder"),
        String::from("skip"),
        String::from("sub"),
        String::from("sum"),
        String::from("swap"),
        String::from("up")
    ];

    let mut tokens: Vec<Token> = Vec::new();

    let chars: Vec<char> = contents.chars().collect();

    let mut idx: usize = 0;
    while idx < chars.len()
    {  
        let mut temp_key: GybeTkn;
        let mut temp_value: String = String::new();

        if chars[idx].is_whitespace()
        {
            if chars[idx] != '\n'
            {
                idx += 1;
                continue;
            }
        }

        if chars[idx].is_numeric()
        {
            while chars[idx].is_numeric()
            {
                temp_value.push(chars[idx]);
                idx += 1;
            }

            tokens.push(Token{key: GybeTkn::LTERAL, value: temp_value});
            continue;
        }

        if chars[idx].is_alphabetic()
        {
            let mut keyword: String = String::new();
            while chars[idx].is_alphabetic()
            {
                keyword.push(chars[idx]);
                idx += 1;
            }

            temp_key = GybeTkn::ILLEGL;
            for string in &keywords
            {
                if &keyword == string
                {
                    temp_key = GybeTkn::KEYWRD;
                    break;
                }
            }
            tokens.push(Token{key: temp_key, value: keyword});
            continue;
        }

        match chars[idx]
        {
            '\'' =>
            {
                while chars[idx + 1] != '\''
                {
                    temp_value.push(chars[idx + 1]);
                    idx += 1;
                }
                idx += 1; // skip closing single quote
                tokens.push(Token{key: GybeTkn::LTERAL, value: temp_value});
                idx += 1; // normal increment
                continue;
            }
            '﹔' =>
            {
                temp_key = GybeTkn::SPRATR;
            }
            '.' =>
            {
                temp_key = GybeTkn::SPRATR;
            }
            '\n' =>
            {
                temp_key = GybeTkn::SPRATR;
            }
            _ =>
            {
                temp_key = GybeTkn::ILLEGL;
            }
        }  
        temp_value.push(chars[idx]);
        tokens.push(Token{ key: temp_key, value: temp_value });
        idx += 1;
    }

    return tokens;
}