#![recursion_limit = "1024"]
#[macro_use] extern crate error_chain;
extern crate num;

use std::str::FromStr;
use std::fmt;
use std::ops;
use std::collections::HashMap;
use num::rational::BigRational;

pub mod errors {
    error_chain! { }
}
use errors::*;


macro_rules! pop_two {
    ($stack:expr) => {
        {
            let a = $stack.pop().expect("Stack is empty");
            let b = $stack.pop().expect("Stack is empty");
            (a, b)
        }
    }
}


#[derive(Debug, Clone)]
enum Value {
    Num(BigRational),
    Str(String),
}
impl FromStr for Value {
    type Err = Error;
    fn from_str(s: &str) -> Result<Value> {
        if let Ok(n) = s.parse::<BigRational>() {
            return Ok(Value::Num(n))
        }
        if let Ok(f) = s.parse::<f64>() {
            return Ok(Value::Num(BigRational::from_float(f).unwrap()))
        }
        Ok(Value::Str(String::from(s)))
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Num(ref n) => {
                write!(f, "<Num: {}>", n)
            }
            &Value::Str(ref s) => {
                write!(f, "<Str: {}>", s)
            }
        }
    }
}
impl ops::Add for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Value {
        use Value::*;
        match self {
            Num(n) => {
                match rhs {
                    Num(n2) => {
                        Num(n + n2)
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}
impl ops::Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Value) -> Value {
        use Value::*;
        match self {
            Num(n) => {
                match rhs {
                    Num(n2) => {
                        Num(n - n2)
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}
impl ops::Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Value {
        use Value::*;
        match self {
            Num(n) => {
                match rhs {
                    Num(n2) => {
                        Num(n * n2)
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}
impl ops::Div for Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Value {
        use Value::*;
        match self {
            Num(n) => {
                match rhs {
                    Num(n2) => {
                        Num(n / n2)
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}


#[derive(Debug, Clone)]
struct Stack {
    buf: Vec<Value>,
}
impl Stack {
    fn new() -> Stack {
        Stack { buf: vec![] }
    }
    fn pop(&mut self) -> Option<Value> {
        self.buf.pop()
    }
    fn last(&self) -> Option<&Value> {
        self.buf.last()
    }
    fn push(&mut self, item: Value) {
        self.buf.push(item);
    }
}
impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut buf_content = self.buf.iter();
        match buf_content.next() {
            Some(item) => s.push_str(&format!("{}", item)),
            _ => return write!(f, "[]"),
        }
        for item in buf_content {
            s.push_str(&format!(", {}", item));
        }
        s.push_str("]");
        write!(f, "{}", s)
    }
}


#[derive(Debug, Clone)]
pub struct Env {
    stack: Stack,
    buffers: HashMap<String, Stack>,
}
impl Env {
    pub fn new() -> Env {
        Env { stack: Stack::new(), buffers: HashMap::new() }
    }
    pub fn eval(&mut self, input: &str) -> Result<()> {
        for token in input.split(' ') {
            match token {
                "f" => println!("{}", self.stack),
                "p" => println!("{}", self.stack.last().unwrap_or(&Value::Str("".into()))),
                "+" => {
                    let (a, b) = pop_two!(self.stack);
                    self.stack.push(a+b);
                }
                "-" => {
                    let (a, b) = pop_two!(self.stack);
                    self.stack.push(b-a);
                }
                "*" => {
                    let (a, b) = pop_two!(self.stack);
                    self.stack.push(a*b);
                }
                "/" => {
                    let (a, b) = pop_two!(self.stack);
                    self.stack.push(b/a);
                }
                _ => {
                    let val = token.parse::<Value>()
                        .expect(&format!("Error parsing token: {:?}", token));
                    self.stack.push(val);
                }
            }
        }
        Ok(())
    }
}
