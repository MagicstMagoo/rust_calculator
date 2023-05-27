// add the following line to your Cargo.toml: lalrpop-util = "0.20.0"
use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;
use std::io::Write;

lalrpop_mod!(pub calculator);

// 定义枚举类型
#[derive(Debug)]
pub enum Expr{
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Var(String),
    Assign(String, Box<Expr>),
}

// 定义函数
pub fn eval(expr: Expr, env:&mut HashMap<String, f64>) -> Result<f64, Sting> {
    match expr{
        // 检测是否为数字
        Expr::Num(n) => Ok(n),
        // 检测为加法 递归地求二者操作数 返回和
        Expr::Add(e1, e2) =>{
            let n1 = eval(*e1, env)?;
            let n2 = eval(*e2, env)?;
            Ok(n1 + n2)
        }

        // 检测为减法 返回差
        Expr::Sub(e1, e2) =>{
            let n1 = eval(*e1, env)?;
            let n2 = eval(*e2, env)?;
            Ok(n1 - n2)
        }

        // 检测为乘法 返回积
        Expr::Mul(e1, e2) =>{
            let n1 = eval(*e1, env)?;
            let n2 = eval(*e2, env)?;
            Ok(n1 * n2)
        }

        // 检测为除法 返回商
        Expr::Div(e1, e2) => {
            let n1 = eval(*e1, env)?;
            let n2 = eval(*e2, env)?;
            if n2 == 0.0 {
                Err("除数不能除零".to_string())
            } else {
                Ok(n1 / n2)
            }
        }

        // 检测为幂运算 返回第一个操作数的第二个操作数次方
        Expr::Pow(e1, e2) =>{
            let n1 = eval(*e1, env)?;
            let n2 = eval(*e2, env)?;
            Ok(n1.powf(n2))
        }
        
        // 检测为变量 环境中查找值 返回值
        Expr::Var(name) =>{
            if let Some(value) = env.get(&name) {
                Ok(*value)
            } else {
                Err(format!("未定义变量: {}", name))
            }
        }

        // 检测为赋值 将值存储在环境中与左边的变量名对应 返回值
        Expr::Assign(name, e) =>{
            let value = eval(*e, env)?;
            env.insert(name, value);
            Ok(value)
        }
    }
}
