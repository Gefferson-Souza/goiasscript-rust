
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

// Implementa métodos para acesso a propriedades e elementos
impl GoiasValue {
    // Método para acessar propriedade de um objeto
    fn get_property(&self, key: &str) -> GoiasValue {
        match self {
            GoiasValue::Object(map) => {
                map.get(key).cloned().unwrap_or(GoiasValue::Null)
            },
            _ => GoiasValue::Null, // Operação inválida para tipos não-objeto
        }
    }
    
    // Método para acessar elemento de um array por índice
    fn get_index(&self, index: &GoiasValue) -> GoiasValue {
        match (self, index) {
            (GoiasValue::Array(arr), GoiasValue::Number(idx)) => {
                let i = *idx as usize;
                if i < arr.len() {
                    arr[i].clone()
                } else {
                    GoiasValue::Null
                }
            },
            (GoiasValue::String(s), GoiasValue::Number(idx)) => {
                let i = *idx as usize;
                if i < s.len() {
                    // Retorna o caractere como string
                    s.chars().nth(i).map_or(GoiasValue::Null, |c| {
                        GoiasValue::String(c.to_string())
                    })
                } else {
                    GoiasValue::Null
                }
            },
            _ => GoiasValue::Null, // Operação inválida
        }
    }
}

use std::io::{self, Write};
fn main() {
    println!    (    "Testando acesso a propriedades e elementos em GoiásScript"    )    ;
    let     pessoa     =     {
        let mut obj = std::collections::HashMap::new();
        obj.insert(String::from("nome"),         GoiasValue::String(String::from("João"))        );
        obj.insert(String::from("idade"),         GoiasValue::Number(30.0)        );
        obj.insert(String::from("habilidades"),         GoiasValue::Array(vec![        GoiasValue::String(String::from("programação"))        ,         GoiasValue::String(String::from("música"))        ,         GoiasValue::String(String::from("culinária"))        ])        );
        GoiasValue::Object(obj)
    }    ;
    println!    (    "Nome: {}"    ,     pessoa    .get_property("nome")    )    ;
    println!    (    "Idade: {}"    ,     pessoa    .get_property("idade")    )    ;
    println!    (    "Primeira habilidade: {}"    ,     pessoa    .get_property("habilidades")    .get_index(&    GoiasValue::Number(0.0)    )    )    ;
    println!    (    "Segunda habilidade: {}"    ,     pessoa    .get_property("habilidades")    .get_index(&    GoiasValue::Number(1.0)    )    )    ;
    println!    (    "Terceira habilidade: {}"    ,     pessoa    .get_property("habilidades")    .get_index(&    GoiasValue::Number(2.0)    )    )    ;
    let     numeros     =     GoiasValue::Array(vec![    GoiasValue::Number(10.0)    ,     GoiasValue::Number(20.0)    ,     GoiasValue::Number(30.0)    ,     GoiasValue::Number(40.0)    ,     GoiasValue::Number(50.0)    ])    ;
    println!    (    "Primeiro número: {}"    ,     numeros    .get_index(&    GoiasValue::Number(0.0)    )    )    ;
    println!    (    "Terceiro número: {}"    ,     numeros    .get_index(&    GoiasValue::Number(2.0)    )    )    ;
    println!    (    "Último número: {}"    ,     numeros    .get_index(&    GoiasValue::Number(4.0)    )    )    ;
    let     texto     =     GoiasValue::String(String::from("GoiásScript"))    ;
    println!    (    "Primeira letra: {}"    ,     texto    .get_index(&    GoiasValue::Number(0.0)    )    )    ;
    println!    (    "Quarta letra: {}"    ,     texto    .get_index(&    GoiasValue::Number(3.0)    )    )    ;
    let     empresa     =     {
        let mut obj = std::collections::HashMap::new();
        obj.insert(String::from("nome"),         GoiasValue::String(String::from("TechGoiás"))        );
        obj.insert(String::from("endereco"),         {
            let mut obj = std::collections::HashMap::new();
            obj.insert(String::from("cidade"),             GoiasValue::String(String::from("Goiânia"))            );
            obj.insert(String::from("estado"),             GoiasValue::String(String::from("Goiás"))            );
            GoiasValue::Object(obj)
        }        );
        obj.insert(String::from("funcionarios"),         GoiasValue::Array(vec![        {
            let mut obj = std::collections::HashMap::new();
            obj.insert(String::from("nome"),             GoiasValue::String(String::from("Maria"))            );
            obj.insert(String::from("cargo"),             GoiasValue::String(String::from("Desenvolvedora"))            );
            GoiasValue::Object(obj)
        }        ,         {
            let mut obj = std::collections::HashMap::new();
            obj.insert(String::from("nome"),             GoiasValue::String(String::from("Pedro"))            );
            obj.insert(String::from("cargo"),             GoiasValue::String(String::from("Designer"))            );
            GoiasValue::Object(obj)
        }        ])        );
        GoiasValue::Object(obj)
    }    ;
    println!    (    "Empresa: {}"    ,     empresa    .get_property("nome")    )    ;
    println!    (    "Cidade: {}"    ,     empresa    .get_property("endereco")    .get_property("cidade")    )    ;
    println!    (    "Estado: {}"    ,     empresa    .get_property("endereco")    .get_property("estado")    )    ;
    println!    (    "Primeiro funcionário: {}"    ,     empresa    .get_property("funcionarios")    .get_index(&    GoiasValue::Number(0.0)    )    .get_property("nome")    )    ;
    println!    (    "Cargo: {}"    ,     empresa    .get_property("funcionarios")    .get_index(&    GoiasValue::Number(0.0)    )    .get_property("cargo")    )    ;
    println!    (    "Teste concluído!"    )    ;
}
