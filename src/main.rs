use std::io::stdin;

#[derive(Debug)]
enum Token
{
    Number(f64),
    RPartentheses,
    LParentheses,
    Exponent,
    Multiplication,
    Division,
    Addition,
    Subtraction,
}

pub struct Calculator
{
    // string input 
    input: String,
    //token position in string
    pos: usize,
    // result of process
    result: f64,
}

impl Calculator
{


}

pub trait Operation
{
    fn new(input: String, pos: usize, result: f64) -> Self;
    fn tokenize(&mut self) -> Vec<Token>;
    fn next_token(&mut self) -> Option<Token>;
    fn skip_whitespace(&mut self);
    fn read_number(&mut self) -> (String, usize);
    fn evaluate(&mut self, tokens: Vec<Token>) -> f64;
}

impl Operation for Calculator
{

    fn new(input: String, pos: usize, result: f64) -> Calculator
    {
        Calculator {input: input, pos, result}
    }

    fn tokenize(&mut self) -> Vec<Token>
    {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()
        {
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> Option<Token>
    {
        self.skip_whitespace();

        if self.pos >= self.input.len()
        {
            return None;
        }

        let current_char = self.input[self.pos..].chars().next().unwrap();

        match current_char
        {
            /*'^' =>
            {
                self.pos += 1;
                println!("^");
                Some(Token::Exponent)
                
            }*/
            '+' => 
            {
                self.pos += 1;
                println!("+");
                Some(Token::Addition)
            }
            '-' =>
            {
                self.pos += 1;
                Some(Token::Subtraction)
            }
            '*' =>
            {
                self.pos += 1;
                Some(Token::Multiplication)
            }
            '/' =>
            {
                self.pos += 1;
                Some(Token::Division)
            }
            '(' =>
            {
                self.pos += 1;
                Some(Token::LParentheses)
            }
            ')' =>
            {
                self.pos +=1;
                Some(Token::RPartentheses)
            }
            '0'..='9' | '.' =>
            {
                let(number_str, new_pos) = self.read_number();
                self.pos = new_pos;
                if let Ok(number) = number_str.parse::<f64>()
                {
                    Some(Token::Number(number))
                }
                else
                {
                    None
                }
            }

            _ =>
            {
                self.pos += 1;
                None
            }
        }

    }

    fn skip_whitespace(&mut self)
    {
        while let Some(c) = self.input[self.pos..].chars().next()
        {
            if !c.is_whitespace()
            {
                break;
            }
            self.pos += c.len_utf8();
        }
    }

    fn read_number(&mut self) -> (String, usize)
    {
        let mut number_str = String::new();
        let mut current_pos = self.pos;

        while let Some(c) = self.input[current_pos..].chars().next()
        {
            if c.is_digit(10) || c == '.'
            {
                number_str.push(c);
                current_pos += c.len_utf8();
            }
            else
            {
                break;
            }
        }
        println!("{}", number_str);
        println!("{}", current_pos);

        (number_str, current_pos)

    }

    fn evaluate(&mut self, tokens: Vec<Token>) -> f64
    {
        self.result = 0.0;

        let mut currrent_op = Token::Addition;

        for token in tokens
        {
            match token
            {

                //Token::Exponent => currrent_op = Token::Exponent,
                Token::Addition => currrent_op = Token::Addition,
                Token::Subtraction => currrent_op = Token::Subtraction,
                Token::Multiplication => currrent_op = Token::Multiplication,
                Token::Division => currrent_op = Token::Division,
                Token::Number(num) =>
                {
                    match currrent_op
                    {
                        Token::Addition => self.result += num,
                        Token::Subtraction => self.result -= num,
                        Token::Multiplication => self.result *= num, 
                        Token::Division => self.result /= num,
                        //Token::Exponent => self.result.pow(num),
                        _ => panic!("Unexpected Token"),
                    }
                }
                _ => panic!("Unexpected token"),
            }
        }
        self.result
    }
}

fn main() 
{
    println!("Calculator");

    let mut vect = String::new();
    let pos: usize = 0;
    let result: f64 = 0.0;

    let mut work: Calculator = Operation::new(vect, pos, result);

    let mut x: String = String::new();

    println!("Enter Equation");
    let _input = stdin().read_line(&mut x).expect("Invalid Input");

    work.input = x;

    let this = work.tokenize();   
    let this1 = work.evaluate(this);

    println!("Result: {}", this1);

}
