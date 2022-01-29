#[test]
fn part1_sample_works() {
    let input = parse_input("1 + 2 * 3 + 4 * 5 + 6");
    let result = sum_exprs(&input);
    assert_eq!(result, 71);

    assert_eq!(evaluate(&parse_expr("2 * 3 + (4 * 5)")), 26);
    assert_eq!(evaluate(&parse_expr("5 + (8 * 3 + 9 + 3 * 4 * 3)")), 437);
    assert_eq!(
        evaluate(&parse_expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
        12240
    );
    assert_eq!(
        evaluate(&parse_expr(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
        )),
        13632
    );
}

#[derive(Debug)]
enum Expression {
    Literal(i32),
    Sum(Box<Expression>, Box<Expression>),
    Prod(Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
enum Token {
    Int(i32),
    Char(char),
}

fn parse_expr_inner_(input: &[Token], i: usize, before: Expression) -> (usize, Expression) {
    //println!("{:?},{},{:?}", input, i, before);
    if i >= input.len() {
        return (i, before);
    }
    match input[i] {
        Token::Int(_) => {
            println!("{:?},{},{:?}", input, i, before);
            panic!("Invalid sequence")
        }
        Token::Char(')') => return (i, before),
        Token::Char(c) => match input[i + 1] {
            Token::Char('(') => {
                let (j, e2) = parse_expr_paren(input, i + 2);
                let e = match c {
                    '+' => Expression::Sum(Box::new(before), Box::new(e2)),
                    '*' => Expression::Prod(Box::new(before), Box::new(e2)),
                    _ => panic!("Invalid operand"),
                };
                parse_expr_inner_(input, j, e)
            }
            Token::Int(v) => {
                let e2 = Expression::Literal(v);
                let e = match c {
                    '+' => Expression::Sum(Box::new(before), Box::new(e2)),
                    '*' => Expression::Prod(Box::new(before), Box::new(e2)),
                    _ => panic!("Invalid operand"),
                };
                parse_expr_inner_(input, i + 2, e)
            }
            _ => panic!("Not a valid start of value"),
        },
    }
}

fn parse_expr_paren(input: &[Token], i: usize) -> (usize, Expression) {
    //println!("{:?},{}", input, i);
    let (j, first) = match input[i] {
        Token::Int(v) => (i + 1, Expression::Literal(v)),
        Token::Char('(') => parse_expr_paren(input, i + 1),
        _ => panic!("Invalid start of subexpression"),
    };
    let (j, e) = parse_expr_inner_(input, j, first);
    match input[j] {
        Token::Char(')') => (j + 1, e),
        _ => panic!("Unmatched paren"),
    }
}

fn parse_expr_inner_top(input: &[Token], i: usize) -> (usize, Expression) {
    //println!("{:?},{}", input, i);
    match input[i] {
        Token::Char('(') => {
            let (j, e) = parse_expr_paren(input, i + 1);
            return parse_expr_inner_(input, j, e);
        }
        Token::Int(v) => {
            let e1 = Expression::Literal(v);
            parse_expr_inner_(input, i + 1, e1)
        }
        _ => panic!("Invalid expression start"),
    }
}

fn parse_expr(input: &str) -> Expression {
    let tokens = input.chars().filter(|c| *c != ' ');
    let tokens = tokens
        .map(|c| {
            if c >= '0' && c <= '9' {
                Token::Int(c.to_digit(10).unwrap() as i32)
            } else {
                Token::Char(c)
            }
        })
        .collect::<Vec<_>>();
    let (j, e) = parse_expr_inner_top(&tokens[..], 0);
    if j != tokens.len() {
        println!("{:?} => {} {:?}", tokens, j, e);
        panic!("Did not consume whole input");
    }
    return e;
}

#[test]
fn parsing_works() {
    assert_eq!(sum_exprs(&parse_input("(1 + 1) + (1 + 1) + 1")), 5);
}

fn parse_input(input: &str) -> Vec<Expression> {
    input
        .trim()
        .split("\n")
        .map(|l| parse_expr(l.trim()))
        .collect()
}

fn evaluate(exp: &Expression) -> i64 {
    //println!("{:?}", exp);
    match exp {
        Expression::Literal(v) => (*v).into(),
        Expression::Sum(e1, e2) => {
            let v1 = evaluate(e1);
            let v2 = evaluate(e2);
            //println!("{:?} = {} + {}",exp, v1, v2);
            v1 + v2
        }
        Expression::Prod(e1, e2) => {
            let v1 = evaluate(e1);
            let v2 = evaluate(e2);
            //println!("{:?} = {} * {}",exp, v1, v2);
            v1 * v2
        }
    }
}

fn sum_exprs(input: &Vec<Expression>) -> i64 {
    input.iter().fold(0, |acc, e| acc + evaluate(e))
}

pub fn part1() -> i64 {
    let input = parse_input(include_str!("inputs/day18.txt"));
    sum_exprs(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 650217205854)
}

fn parse_expr2_infix(input: &[Token], i: usize, before: Expression) -> (usize, Expression) {
    if i == input.len() {
        return (i, before);
    }
    match input[i] {
        Token::Char(')') => (i, before),
        Token::Char('+') => {
            /* A + B [...], strongest bind so do it now */
            let (j, e) = parse_expr2_operand(input, i+1);
            let new = Expression::Sum(Box::new(before), Box::new(e));
            parse_expr2_infix(input, j, new)
        }
        Token::Char('*') => {
            /*
                A * B [...], the B might be part of something else
                so if we parse B [...] to an expresision F then
                A * F is the correct parsing
                probably
             */
            let (j,e) = parse_expr2_operand(input, i+1);
            let (k,e) = parse_expr2_infix(input, j, e);
            (k, Expression::Prod(Box::new(before), Box::new(e)))
        }
        _ => panic!("Expected end of expression or operand"),
    }
}

fn parse_expr2_operand(input: &[Token], i: usize) -> (usize, Expression) {
    match input[i] {
        Token::Char('(') => {
            let (j, e) = parse_expr2_top(input, i + 1);
            match input[j] {
                Token::Char(')') => (j + 1, e),
                _ => panic!("Unmatched paren"),
            }
        }
        Token::Int(v) => {
            let e = Expression::Literal(v);
            (i + 1, e)
        }
        _ => panic!("Invalid expression start"),
    }
}

fn parse_expr2_top(input: &[Token], i: usize) -> (usize, Expression) {
    let (j, e1) = parse_expr2_operand(input, i);
    parse_expr2_infix(input, j, e1)
}

fn parse_expr2(input: &str) -> Expression {
    let tokens = input.chars().filter(|c| *c != ' ');
    let tokens = tokens
        .map(|c| {
            if c >= '0' && c <= '9' {
                Token::Int(c.to_digit(10).unwrap() as i32)
            } else {
                Token::Char(c)
            }
        })
        .collect::<Vec<_>>();
    let (j, e) = parse_expr2_top(&tokens[..], 0);
    if j != tokens.len() {
        println!("{:?} => {} {:?}", tokens, j, e);
        panic!("Did not consume whole input");
    }
    return e;
}

fn parse_input2(input: &str) -> Vec<Expression> {
    input
        .trim()
        .split("\n")
        .map(|l| parse_expr2(l.trim()))
        .collect()
}

#[test]
fn part2_sample_works() {
    let e = parse_expr2("1 + (2 * 3) + (4 * (5 + 6))");
    assert_eq!(evaluate(&e), 51);

    let e = parse_expr2("2 * 3 + (4 * 5)");
    println!("{:?}", e);
    assert_eq!(evaluate(&e), 46);
}

pub fn part2() -> i64 {
    let input = parse_input2(include_str!("inputs/day18.txt"));
    sum_exprs(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 20394514442037)
}