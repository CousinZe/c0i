pub mod callable;
pub mod result;
pub mod scope;

use std::{collections::HashMap, convert::identity, fmt::Display, sync::{Arc, RwLock}};

use callable::Callable;
use sexpr_ir::gast::{symbol::Symbol, Handle};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Char(char),
    Uint(u64),
    Int(i64),
    Float(f64),
    Str(Handle<String>),
    Sym(Handle<Symbol>),
    Pair(Handle<Pair>),
    Dict(Dict),
    Vec(Vector),
    Callable(Callable),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Nil, Value::Nil) => true.partial_cmp(&true),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::Char(a), Value::Char(b)) => a.partial_cmp(b),
            (Value::Uint(a), Value::Uint(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Str(a), Value::Str(b)) => a.partial_cmp(b),
            (Value::Sym(a), Value::Sym(b)) => a.0.partial_cmp(&b.0),
            (Value::Pair(_a), Value::Pair(_b)) => None,
            (Value::Dict(_a), Value::Dict(_b)) => None,
            (Value::Vec(a), Value::Vec(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(v) => write!(f, "{}", v),
            // Value::Char(v) => write!(f, "'{}'", v),
            Value::Uint(v) => write!(f, "{}", v),
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "\"{}\"", v),
            Value::Sym(v) => write!(f, "{}", v),
            Value::Char(v) => write!(f, "(char \"{}\")", v),
            Value::Dict(v) => v.fmt(f),
            Value::Vec(v) => v.fmt(f),
            Value::Pair(v) => v.fmt(f),
            Value::Callable(v) => v.fmt(f),
        }
    }
}


impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut this = self;
        write!(f, "'(")?;
        let mut start = true;
        loop {
            match this {
                Pair(v, Value::Pair(t)) => {
                    if !start {
                        write!(f, " ")?;
                    }
                    v.fmt(f)?;
                    this = t;
                    start = false;
                    continue;
                },
                Pair(v, Value::Nil) => {
                    if !start {
                        write!(f, " ")?;
                    }
                    v.fmt(f)?;
                    break;
                },
                Pair(v, t) => {
                    if !start {
                        write!(f, " ")?;
                    }
                    v.fmt(f)?;
                    write!(f, " . ")?;
                    t.fmt(f)?;
                    break;
                },
            }
        }
        write!(f, ")")
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.0.read().unwrap()
        .iter()
        .map(|x| x.read().unwrap().to_string())
        .collect::<Vec<_>>();
        write!(f, "(vec {})", r.join(" "))
    }
}

impl Display for Dict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.0.read().unwrap().iter()
            .map(|(k, v)| format!("'(\"{}\" . {})", k, v))
            .collect::<Vec<_>>();
        write!(f, "(dict {})", r.join(" "))
    }
}


macro_rules! impl_is_type {
    ($name:ident, $tp:ident) => {
        pub fn $name(&self) -> bool {
            matches!(self, Value::$tp(_))
        }
    };
}


impl Value {
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }
    impl_is_type!(is_bool, Bool);
    impl_is_type!(is_char, Char);
    impl_is_type!(is_int, Int);
    impl_is_type!(is_uint, Uint);
    impl_is_type!(is_float, Float);
    impl_is_type!(is_str, Str);
    impl_is_type!(is_sym, Sym);
    impl_is_type!(is_pair, Pair);
    impl_is_type!(is_dict, Dict);
    impl_is_type!(is_vec, Vec);
    impl_is_type!(is_callable, Callable);
}


#[derive(Debug, Clone, PartialEq)]
pub struct Pair(pub Value, pub Value);

#[derive(Debug, Clone, Default)]
pub struct Dict(pub Arc<RwLock<HashMap<Handle<String>, Value>>>);

impl PartialEq for Dict {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct Vector(pub Arc<RwLock<Vec<RwLock<Value>>>>);

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.0.read().unwrap().iter().zip(other.0.read().unwrap().iter())
            .map(|(a, b)| *a.read().unwrap() == *b.read().unwrap())
            .reduce(|a, b| a && b).map_or(false, identity)
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}


impl From<&[Value]> for Value {
    fn from(i: &[Value]) -> Self {
        if let Some(left) = i.first() {
            let right = Value::from(&i[1..]);
            Value::Pair(Handle::new(Pair(left.clone(), right)))
        } else {
            Value::Nil
        }
    }
}