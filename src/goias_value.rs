use std::fmt;

/// Representa qualquer valor válido no GoiásScript
#[derive(Debug, Clone)]
pub enum GoiasValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<GoiasValue>),
    Null,
}

impl fmt::Display for GoiasValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl From<f64> for GoiasValue {
    fn from(value: f64) -> Self {
        GoiasValue::Number(value)
    }
}

impl From<String> for GoiasValue {
    fn from(value: String) -> Self {
        GoiasValue::String(value)
    }
}

impl From<&str> for GoiasValue {
    fn from(value: &str) -> Self {
        GoiasValue::String(value.to_string())
    }
}

impl From<bool> for GoiasValue {
    fn from(value: bool) -> Self {
        GoiasValue::Boolean(value)
    }
}

impl<T: Into<GoiasValue>> From<Vec<T>> for GoiasValue {
    fn from(values: Vec<T>) -> Self {
        GoiasValue::Array(values.into_iter().map(|v| v.into()).collect())
    }
}