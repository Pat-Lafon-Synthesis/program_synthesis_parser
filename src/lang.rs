use std::fmt::Display;

use hashconsing::{consign, hcons, HConsed, HashConsign};

#[derive(Debug)]
pub struct SynthProblem {
    pub imports: Vec<String>,
    pub decls: Vec<Declaration>,
    pub synth_type: Type,
    pub spec: Problem,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
pub enum Pattern {
    Tuple(Vec<Pattern>),
    Ctor(String, Box<Pattern>),
    Var(String),
    Wildcard,
}

impl Pattern {
    pub fn contains_var(&self, id: &str) -> bool {
        match self {
            Pattern::Tuple(v) => v.iter().any(|p| p.contains_var(id)),
            Pattern::Ctor(_, p) => p.contains_var(id),
            Pattern::Var(id2) => id == id2,
            Pattern::Wildcard => false,
        }
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Tuple(id_list) => {
                write!(
                    f,
                    "{}",
                    id_list
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Pattern::Ctor(id, p) => write!(f, "{id} {p}"),
            Pattern::Var(id) => write!(f, "{id}"),
            Pattern::Wildcard => write!(f, "_"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Declaration {
    TypeDecl(String, Type),
    ExpDecl(String, Exp),
}

#[hcons(name = "Type", impls = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum ActualType {
    Named(String),
    Arrow(Type, Type),
    Tuple(Vec<Type>),
    Mu(String, Type),
    Variant(Vec<(String, Type)>),
}

impl ActualType {
    pub fn is_named(&self) -> bool {
        match self {
            Self::Named(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Problem {
    UIOEs(Vec<Example>),
    UEquiv(Vec<Example>, Exp),
    UPost(Exp),
}

#[derive(Debug)]
pub struct Example {
    pub input: Vec<Exp>,
    pub output: Exp,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[hcons(name = "Exp", impls = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum ActualExp {
    Var(String),
    Wildcard,
    App(Exp, Exp),
    Func(Param, Exp),
    CTor(String, Exp),
    Unctor(String, Exp),
    Eq(bool, Exp, Exp),
    Match(Exp, Vec<(Pattern, Exp)>),
    Fix(String, Type, Exp),
    Tuple(Vec<Exp>),
    Proj(i64, Exp),
}

impl Exp {
    pub fn mk_unctor_or_ctor_by_name(c: &str, e: Exp) -> Self {
        match c.strip_prefix("Un_") {
            Some(x) => Self::Unctor(x.to_string(), e),
            None => Self::CTor(c.to_string(), e),
        }
    }
    pub fn Unit() -> Self {
        Self::Tuple(Vec::new())
    }
}

impl From<u64> for Exp {
    fn from(mut value: u64) -> Self {
        let mut inital_val = Exp::CTor("O".to_string(), Exp::Unit());
        while value > 0 {
            inital_val = Exp::CTor("S".to_string(), inital_val);
            value -= 1;
        }
        inital_val
    }
}
