
// Representa qualquer valor válido no GoiásScript
#[derive(Debug, Clone)]
enum GoiasValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<GoiasValue>),
    Object(std::collections::HashMap<String, GoiasValue>),
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
            },
            GoiasValue::Object(props) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in props {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                    first = false;
                }
                write!(f, "}}")
            },
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
    println!    (    "Testando objetos em GoiásScript"    )    ;
    let     objeto_vazio     =     {
        let mut obj = std::collections::HashMap::new();
        GoiasValue::Object(obj)
    }    ;
    println!    (    "Objeto vazio: {}"    ,     objeto_vazio    )    ;
    let     pessoa     =     {
        let mut obj = std::collections::HashMap::new();
        obj.insert(String::from("nome"),         GoiasValue::String(String::from("João"))        );
        obj.insert(String::from("idade"),         GoiasValue::Number(30.0)        );
        obj.insert(String::from("ativo"),         GoiasValue::Boolean(true)        );
        GoiasValue::Object(obj)
    }    ;
    println!    (    "Pessoa: {}"    ,     pessoa    )    ;
    let     dados     =     {
        let mut obj = std::collections::HashMap::new();
        obj.insert(String::from("id"),         GoiasValue::Number(1001.0)        );
        obj.insert(String::from("titulo"),         GoiasValue::String(String::from("Produto Teste"))        );
        obj.insert(String::from("preco"),         GoiasValue::Number(29.99)        );
        obj.insert(String::from("disponivel"),         GoiasValue::Boolean(true)        );
        obj.insert(String::from("categorias"),         GoiasValue::Array(vec![        GoiasValue::String(String::from("eletrônicos"))        ,         GoiasValue::String(String::from("promoção"))        ])        );
        GoiasValue::Object(obj)
    }    ;
    println!    (    "Dados completos: {}"    ,     dados    )    ;
    let     contador     =     GoiasValue::Number(10.0)    ;
    let     config     =     {
        let mut obj = std::collections::HashMap::new();
        obj.insert(String::from("min"),         &        contador         - &        GoiasValue::Number(5.0)        );
        obj.insert(String::from("max"),         &        contador         + &        GoiasValue::Number(5.0)        );
        obj.insert(String::from("valor_padrao"),         contador        );
        GoiasValue::Object(obj)
    }    ;
    println!    (    "Configuração: {}"    ,     config    )    ;
    println!    (    "Teste de objetos concluído!"    )    ;
}
