#[derive(PartialEq)]
#[derive(Debug)]
pub enum GybeTkn
{
    NUM,
    IDEN,

    QUOTE,
    SEMI,
    PERIOD,

    ILLEGAL
}

pub struct Token
{
    pub key: GybeTkn,
    pub value: String,
}

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
        GybeTkn::ILLEGAL => s = "ILLEGAL",
    }

    println!("{} | {}", s, t.value);
}

pub fn lex(contents: String) -> Vec<Token>
{
    let mut tokens: Vec<Token> = Vec::new();

    let lower_contents: String = contents.to_ascii_lowercase();
    let chars: Vec<char> = lower_contents.chars().collect();

    let mut idx: usize = 0;
    while idx < chars.len()
    {  
        let temp_key: GybeTkn;
        let mut temp_value: String = String::new();

        let next: char = chars[idx];

        if next.is_ascii_whitespace() || next.is_ascii_control()
        {
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