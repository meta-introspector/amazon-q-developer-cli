use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Var(String),
    App(Box<Expr>, Box<Expr>),
    Lam(String, Box<Expr>),
    S,
    K,
    I,
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Var(name) => name.clone(),
            Expr::App(f, x) => format!("({} {})", f.to_string(), x.to_string()),
            Expr::Lam(var, body) => format!("(λ{}.{})", var, body.to_string()),
            Expr::S => "S".to_string(),
            Expr::K => "K".to_string(),
            Expr::I => "I".to_string(),
        }
    }
}
