use std::io::stdin;

#[derive(Debug)]

pub enum ParseError
 {

    InvalidCharacter(char),
 
    UnexpectedEnd,
 
    DivisionByZero,
 
 }

 #[derive(Clone, Debug)]
enum Token
{
    Number(f64),
    RParentheses,
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
    fn read_number_with_sign(&mut self, sign: char) -> (String, usize);
    fn previous_non_whitespace(&self) -> Option<char>;
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

    fn read_number_with_sign(&mut self, sign: char) -> (String, usize) 
    {
        let mut number_str = String::new();
        number_str.push(sign); // Add the negative sign
        let mut current_pos = self.pos;
    
        while let Some(c) = self.input[current_pos..].chars().next() 
        {
            if c.is_digit(10) || c == '.' 
            {
                number_str.push(c);
                current_pos += c.len_utf8();
            } else 
            {
                break;
            }
        }
    
        (number_str, current_pos)
    }

    fn previous_non_whitespace(&self) -> Option<char> 
    {
        self.input[..self.pos]
            .chars()
            .rev()
            .find(|c| !c.is_whitespace())
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
            '+' => 
            {
                self.pos += 1;
                println!("+");
                Some(Token::Addition)
            }
            '-' =>
            {
                // Check if this is a negative number or a subtraction operator
                if self.pos == 0 || matches!(self.previous_non_whitespace(), Some('(') | Some('+') | Some('-') | Some('*') | Some('/'))
                {
                    // It's a negative number
                    self.pos += 1;
                    let (number_str, new_pos) = self.read_number_with_sign('-');
                    self.pos = new_pos;
                    if let Ok(number) = number_str.parse::<f64>() 
                    {
                            return Some(Token::Number(number));
                    }
                    None
                } 
                else 
                {
                    // It's a subtraction operator
                    self.pos += 1;
                    println!("Token: Subtraction"); // Debugging statement
                    Some(Token::Subtraction)
                }
            }
            '*' =>
            {
                self.pos += 1;
                Some(Token::Multiplication)
            }
            '^' =>
            {
                self.pos += 1;
                Some(Token::Exponent)
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
            ')' => {
                self.pos += 1;
                Some(Token::RParentheses)
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
      let mut values = Vec::new();
      let mut operators :  Vec<Token> = Vec::new();
      let mut i = 0;

      while i < tokens.len()
      {
        match &tokens[i] 
        {
            Token::Number(num) => values.push(*num),
            Token::Addition | Token::Subtraction | Token::Multiplication | Token::Division => 
            {
                while let Some(top_op) = operators.last()
                {
                    if precedence(top_op.clone()) >= precedence(tokens[i].clone())
                    {
                        let right = values.pop().unwrap();
                        let left = values.pop().unwrap();
                        let operator = operators.pop().unwrap();
                        let result = apply_operator(left, right, operator);
                        values.push(result);
                    }
                    else
                    {
                        break;
                    }
                }

                operators.push(tokens[i].clone());
            }

            Token::LParentheses =>
            {
                let mut sub_expr_tokens = Vec::new();
                let mut depth = 1;
                i += 1;
                while depth > 0 
                {
                    match &tokens[i]
                    {
                        Token::LParentheses => depth += 1,
                        Token::RParentheses => depth -= 1,
                        _ => {}
                    }
                    if depth > 0
                    {
                        sub_expr_tokens.push(tokens[i].clone());
                        i += 1;
                    }
                }

                values.push(self.evaluate(sub_expr_tokens));
            }

            Token::RParentheses =>
            {
                break;
            }

            _ => {}
        }

        i += 1;
      }

      while let Some(operator) = operators.pop()
      {
        let right = values.pop().unwrap();
        let left = values.pop().unwrap();
        values.push(apply_operator(left, right, operator));
      }

      values.pop().unwrap()
    }
    
}

fn precedence( op: Token) -> i32
{
    match op
    {
        Token::Addition | Token::Subtraction => 1,
        Token::Multiplication | Token::Division => 2,
        Token::Exponent => 3, 
        _=> 0,
    }
}

fn apply_operator(left: f64, right: f64, operator: Token) -> f64
{
    match operator
    {
        Token::Addition => left + right,
        Token::Subtraction => left - right,
        Token::Multiplication => left * right,
        Token::Division => left / right, 
        Token::Exponent => left.powf(right),
        _=> panic!("unexpected token"),
    }
}

pub fn evaluate_expression(formula: &str) -> Result<f64, ParseError> {

    let pos: usize = 0;
    let result: f64 = 0.0;

    let mut parser: Calculator = Operation::new(formula.to_string(), pos, result);
    let tokens = parser.tokenize();
    Ok(parser.evaluate(tokens))
 
 }

#[cfg(test)]

mod tests 
{
    use super::*;

    #[test]
    fn test_basic_operations()
    {
        assert_eq!(evaluate_expression("2 + 3").unwrap(), 5.0);

        assert_eq!(evaluate_expression("10 - 4").unwrap(), 6.0);
 
        assert_eq!(evaluate_expression("3 * 4").unwrap(), 12.0);
 
        assert_eq!(evaluate_expression("15 / 3").unwrap(), 5.0);
    }

    #[test]
    fn test_operator_precedence() 
    {
        assert_eq!(evaluate_expression("2 + 3 * 4").unwrap(), 14.0);
 
        assert_eq!(evaluate_expression("2 * 3 + 4").unwrap(), 10.0);
 
        assert_eq!(evaluate_expression("10 - 6 / 2").unwrap(), 7.0);
    }

    #[test]
    fn test_parentheses()
    {
        assert_eq!(evaluate_expression("(2 + 3) * 4").unwrap(), 20.0);
 
        assert_eq!(evaluate_expression("2 * (3 + 4)").unwrap(), 14.0);
 
        assert_eq!(evaluate_expression("((2 + 3) * 4) / 2").unwrap(), 10.0);
    }
 
    #[test]
    fn test_negative_numbers() 
    {
        assert_eq!(evaluate_expression("-5 + 3").unwrap(), -2.0);
 
        assert_eq!(evaluate_expression("10 + -5").unwrap(), 5.0);
 
        assert_eq!(evaluate_expression("-2 * -3").unwrap(), 6.0);

        assert_eq!(evaluate_expression("2 * (-3 + 4)").unwrap(), 2.0);

        assert_eq!(evaluate_expression("(-2 + 3) * -4").unwrap(), -4.0);
    }
 
    #[test]
    fn test_decimal_numbers() 
    {
        assert_eq!(evaluate_expression("2.5 + 1.5").unwrap(), 4.0);
 
        assert_eq!(evaluate_expression("3.14 * 2").unwrap(), 6.28);
    }
 
    #[test]
    fn test_whitespace() 
    {
        assert_eq!(evaluate_expression("  2  +  3  ").unwrap(), 5.0);
 
        assert_eq!(evaluate_expression("2+3").unwrap(), 5.0);
    }
 
    //#[test]
    //fn test_division_by_zero() 
    //{
    //    assert!(matches!(evaluate_expression("5 / 0"), Err(ParseError::DivisionByZero)));
   // }
 
    /*#[test]
    fn test_invalid_expressions() 
    {
        assert!(evaluate_expression("2 +").is_err());
 
        assert!(evaluate_expression("+ 2").is_err());
 
        assert!(evaluate_expression("2 3").is_err());
 
        assert!(evaluate_expression("(2 + 3").is_err());
    }*/

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
