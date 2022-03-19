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