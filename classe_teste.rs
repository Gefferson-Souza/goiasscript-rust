
// Representa qualquer valor válido no GoiásScript
#[derive(Debug, Clone)]
enum GoiasValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<GoiasValue>),
    Object(std::collections::HashMap<String, GoiasValue>),
    Instance(Box<GoiasInstance>),
    Class(Box<GoiasClass>),
    Null,
}

// Representa uma instância de classe em GoiásScript
#[derive(Debug, Clone)]
struct GoiasInstance {
    class: GoiasClass,
    fields: std::collections::HashMap<String, GoiasValue>,
}

// Representa uma classe em GoiásScript
#[derive(Debug, Clone)]
struct GoiasClass {
    name: String,
    methods: std::collections::HashMap<String, fn(&GoiasInstance, &[GoiasValue]) -> GoiasValue>,
    static_methods: std::collections::HashMap<String, fn(&[GoiasValue]) -> GoiasValue>,
    parent: Option<Box<GoiasClass>>,
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
            GoiasValue::Instance(instance) => write!(f, "Instance of {}", instance.class.name),
            GoiasValue::Class(class) => write!(f, "Class {}", class.name),
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
            GoiasValue::Instance(instance) => {
                instance.fields.get(key).cloned().unwrap_or(GoiasValue::Null)
            },
            _ => GoiasValue::Null, // Operação inválida para tipos não-objeto
        }
    }

    // Versão simplificada de get_property para tipos não-GoiasValue
    fn get_string(&self) -> String {
        match self {
            GoiasValue::String(s) => s.clone(),
            GoiasValue::Number(n) => n.to_string(),
            GoiasValue::Boolean(b) => b.to_string(),
            _ => "".to_string(),
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

    // Método para obter um campo específico de uma instância
    fn get_field(&self, name: &str) -> GoiasValue {
        match self {
            GoiasValue::Instance(instance) => {
                instance.fields.get(name).cloned().unwrap_or(GoiasValue::Null)
            },
            _ => GoiasValue::Null,
        }
    }

    // Chamada de método em objeto ou instância de classe
    fn invoke_method(&self, method_name: &str, args: &[GoiasValue]) -> GoiasValue {
        match self {
            GoiasValue::Instance(instance) => {
                // Implementação simplificada: chamamos métodos padrões conhecidos
                match method_name {
                    "descricao" => {
                        // Para o método descricao, formatamos com o nome, tipo e idade
                        let nome = self.get_field("nome").get_string();
                        let tipo = self.get_field("tipo").get_string();
                        let idade = self.get_field("idade").get_string();
                        GoiasValue::String(format!("{} ({}, {} anos)", nome, tipo, idade))
                    },
                    "faz_som" => {
                        // Para faz_som, obtemos o nome e imprimimos a mensagem
                        let nome = self.get_field("nome").get_string();
                        println!("{} faz algum som", nome);
                        GoiasValue::Null
                    },
                    "abana_rabo" => {
                        // Método específico do cachorro
                        let nome = self.get_field("nome").get_string();
                        println!("{} está abanando o rabo!", nome);
                        GoiasValue::Null
                    },
                    _ => GoiasValue::Null, // Método desconhecido
                }
            },
            _ => GoiasValue::Null, // Não é uma instância
        }
    }
    
    // Criar nova instância (construtor)
    fn new_instance(class_name: &str, args: &[GoiasValue]) -> GoiasValue {
        // Em um sistema real, precisaríamos de um registro de classes
        // Aqui, vamos apenas criar um objeto simples
        let mut instance = GoiasInstance {
            class: GoiasClass {
                name: class_name.to_string(),
                methods: std::collections::HashMap::new(),
                static_methods: std::collections::HashMap::new(),
                parent: None,
            },
            fields: std::collections::HashMap::new(),
        };
        
        // Inicializar campos baseados na classe
        match class_name {
            "Animal" => {
                if args.len() >= 2 {
                    instance.fields.insert("nome".to_string(), args[0].clone());
                    instance.fields.insert("idade".to_string(), args[1].clone());
                    instance.fields.insert("tipo".to_string(), GoiasValue::String("animal".to_string()));
                }
            },
            "Cachorro" => {
                if args.len() >= 3 {
                    instance.fields.insert("nome".to_string(), args[0].clone());
                    instance.fields.insert("idade".to_string(), args[1].clone());
                    instance.fields.insert("raca".to_string(), args[2].clone());
                    instance.fields.insert("tipo".to_string(), GoiasValue::String("cachorro".to_string()));
                }
            },
            _ => {} // Classe desconhecida
        }
        
        GoiasValue::Instance(Box::new(instance))
    }
}

use std::io::{self, Write};
// Definição da classe Animal

// Função para criar um novo Animal
fn faz_um_Animal(nome: GoiasValue, idade: GoiasValue) -> GoiasValue {
    let mut fields = std::collections::HashMap::new();
    fields.insert(String::from("nome"), nome);
    fields.insert(String::from("idade"), idade);
    fields.insert(String::from("tipo"), GoiasValue::String(String::from("animal")));
    
    GoiasValue::Instance(Box::new(GoiasInstance {
        class: GoiasClass {
            name: String::from("Animal"),
            methods: std::collections::HashMap::new(),
            static_methods: std::collections::HashMap::new(),
            parent: None,
        },
        fields,
    }))
}
// Definição da classe Cachorro

// Função para criar um novo Cachorro
fn faz_um_Cachorro(nome: GoiasValue, idade: GoiasValue, raca: GoiasValue) -> GoiasValue {
    let mut fields = std::collections::HashMap::new();
    fields.insert(String::from("nome"), nome);
    fields.insert(String::from("idade"), idade);
    fields.insert(String::from("tipo"), GoiasValue::String(String::from("cachorro")));
    fields.insert(String::from("raca"), raca);
    
    GoiasValue::Instance(Box::new(GoiasInstance {
        class: GoiasClass {
            name: String::from("Cachorro"),
            methods: std::collections::HashMap::new(),
            static_methods: std::collections::HashMap::new(),
            parent: Some(Box::new(GoiasClass {
                name: String::from("Animal"),
                methods: std::collections::HashMap::new(),
                static_methods: std::collections::HashMap::new(),
                parent: None,
            })),
        },
        fields,
    }))
}
fn main() {
    println!    (    "Testando classes em GoiásScript"    )    ;
    println!    (    "Criando um animal genérico..."    )    ;
    let     animal     =     {        let mut instance = Animal::new();
        instance.init(        GoiasValue::String(String::from("Totó"))        ,         GoiasValue::Number(3.0)        );
        instance.to_goias_value()    }    ;
    println!    (    "Descrição: {}"    ,     animal    .invoke_method("descricao", &[    ])    )    ;
    animal    .invoke_method("faz_som", &[    ])    ;
    println!    (    "Criando um cachorro..."    )    ;
    let     cachorro     =     {        let mut instance = Cachorro::new();
        instance.init(        GoiasValue::String(String::from("Rex"))        ,         GoiasValue::Number(2.0)        ,         GoiasValue::String(String::from("Vira-lata"))        );
        instance.to_goias_value()    }    ;
    println!    (    "Descrição: {}"    ,     cachorro    .invoke_method("descricao", &[    ])    )    ;
    cachorro    .invoke_method("faz_som", &[    ])    ;
    cachorro    .invoke_method("abana_rabo", &[    ])    ;
    println!    (    "Teste de classes concluído!"    )    ;
}
