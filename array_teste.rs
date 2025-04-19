
// Representa qualquer valor válido no GoiásScript
#[derive(Debug, Clone)]
enum GoiasValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<GoiasValue>),
    Null,
}

impl std::fmt::Display for GoiasValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GoiasValue::Number(n) => write!(f, "{}", n),
            GoiasValue::String(s) => write!(f, "{}", s),
            GoiasValue::Boolean(b) => write!(f, "{}", b),
            GoiasValue::Array(elements) => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            GoiasValue::Null => write!(f, "null"),
        }
    }
}

// Implementa operações aritméticas para GoiasValue
impl std::ops::Add for &GoiasValue {
    type Output = GoiasValue;

    fn add(self, other: &GoiasValue) -> GoiasValue {
        match (self, other) {
            (GoiasValue::Number(a), GoiasValue::Number(b)) => GoiasValue::Number(a + b),
            (GoiasValue::String(a), b) => GoiasValue::String(format!("{}{}", a, b)),
            (a, GoiasValue::String(b)) => GoiasValue::String(format!("{}{}", a, b)),
            _ => GoiasValue::Null, // Operação inválida
        }
    }
}

impl std::ops::Sub for &GoiasValue {
    type Output = GoiasValue;

    fn sub(self, other: &GoiasValue) -> GoiasValue {
        match (self, other) {
            (GoiasValue::Number(a), GoiasValue::Number(b)) => GoiasValue::Number(a - b),
            _ => GoiasValue::Null, // Operação inválida
        }
    }
}

// Operações para valores owned
impl std::ops::Add for GoiasValue {
    type Output = GoiasValue;

    fn add(self, other: GoiasValue) -> GoiasValue {
        &self + &other
    }
}

impl std::ops::Sub for GoiasValue {
    type Output = GoiasValue;

    fn sub(self, other: GoiasValue) -> GoiasValue {
        &self - &other
    }
}

// Implementa conversão de inteiros para GoiasValue::Number
impl From<i32> for GoiasValue {
    fn from(value: i32) -> Self {
        GoiasValue::Number(value as f64)
    }
}


use std::io::{self, Write};
fn main() {
    println!    (    "Testando arrays em GoiásScript"    )    ;
    let     array_vazio     =     GoiasValue::Array(vec![    ])    ;
    println!    (    "Array vazio: {}"    ,     array_vazio    )    ;
    let     numeros     =     GoiasValue::Array(vec![    GoiasValue::Number(1.0)    ,     GoiasValue::Number(2.0)    ,     GoiasValue::Number(3.0)    ,     GoiasValue::Number(4.0)    ,     GoiasValue::Number(5.0)    ])    ;
    println!    (    "Array de números: {}"    ,     numeros    )    ;
    let     nomes     =     GoiasValue::Array(vec![    GoiasValue::String(String::from("João"))    ,     GoiasValue::String(String::from("Maria"))    ,     GoiasValue::String(String::from("Pedro"))    ])    ;
    println!    (    "Array de nomes: {}"    ,     nomes    )    ;
    let     misturado     =     GoiasValue::Array(vec![    GoiasValue::Number(1.0)    ,     GoiasValue::String(String::from("dois"))    ,     GoiasValue::Boolean(true)    ,     GoiasValue::Number(4.5)    ])    ;
    println!    (    "Array misturado: {}"    ,     misturado    )    ;
    let     contador     =     GoiasValue::Number(10.0)    ;
    let     expressoes     =     GoiasValue::Array(vec![    &    contador     + &    GoiasValue::Number(5.0)    ,     &    contador     - &    GoiasValue::Number(5.0)    ,     contador    ])    ;
    println!    (    "Array de expressões: {}"    ,     expressoes    )    ;
    println!    (    "Teste concluído!"    )    ;
}
