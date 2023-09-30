use std::io;

enum Token {
    Op(char),
    Num(f64),
}

// 将输入的字符串转换为Token类型的向量
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut number = String::new();
    for c in input.chars() {
        // 忽略空格
        if c == ' ' {
            continue;
        }
        if c.is_digit(10) || c == '.' {
            number.push(c);
        } else {
            if !number.is_empty() {
                tokens.push(Token::Num(number.parse().unwrap()));
                number.clear();
            }
            tokens.push(Token::Op(c));
        }
    }
    if !number.is_empty() {
        tokens.push(Token::Num(number.parse().unwrap()));
    }
    tokens
}

// 比较运算符的优先级
fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => panic!("Invalid operator"),
    }
}

// 定义一个函数，根据运算符和两个操作数计算结果，并返回f64类型的值
fn evaluate(op: char, a: f64, b: f64) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        '^' => a.powf(b),
        _ => panic!("Invalid operator"),
    }
}

// 将Token向量转换为逆波兰表达式并返回Token向量
fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut rpn = Vec::new();
    let mut stack = Vec::new();
    for token in tokens {
        match token {
            Token::Num(n) => rpn.push(Token::Num(n)),
            Token::Op(c) => {
                while let Some(Token::Op(top)) = stack.last() {
                    if precedence(*top) >= precedence(c) {
                        rpn.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(Token::Op(c));
            }
        }
    }
    while let Some(token) = stack.pop() {
        rpn.push(token);
    }
    rpn
}

// 根据逆波兰表达式计算结果 返回一个f64类型的值
fn calculate(rpn: Vec<Token>) -> f64 {
    let mut stack = Vec::new();
    for token in rpn {
        match token {
            Token::Num(n) => stack.push(n),
            Token::Op(c) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(evaluate(c, a, b));
            }
        }
    }
    if stack.len() == 1 {
        stack.pop().unwrap()
    } else {
        // 否则抛出异常
        panic!("Invalid expression");
    }
}

fn main() {
    // 创建循环 计算结果
    loop {
        println!("Enter the equation or enter 'quit' to exit:");
        // 创建一个空的字符串，用于存储用户的输入
        let mut input = String::new();
        // 从标准输入读取一行数据 并将其存入input字符串中 如果读取失败，则打印错误信息并退出程序
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        input = input.trim().to_string();
        // 如果输入的字符串为空 跳出本次循环 进行下一次循环
        if input.is_empty() {
            continue;
        }
        if input == "quit" {
            break;
        }
        let tokens = tokenize(&input);
        let rpn = to_rpn(tokens);
        println!("The result is: {}", calculate(rpn));
    }
}
