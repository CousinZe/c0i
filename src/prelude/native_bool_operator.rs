use lazy_static::lazy_static;

use sexpr_ir::gast::{Handle, symbol::Symbol};

use crate::impl_wrap;
use crate::value::Value;
use crate::value::callable::NativeFunction;
use crate::value::result::{CResult, CError};

use super::LOCATION;


fn native_bool_not(args: Vec<Value>) -> CResult {
    if args.len() != 1 {
        return Err(CError::ArgsNotMatching(1, args.len()));
    }
    let v = args.get(0).unwrap();
    if let Value::Bool(b) = v {
        Ok(Value::Bool(!b))
    } else {
        Err(CError::TypeError((), v.clone()))
    }
}

impl_wrap!(BOOL_NOT_WRAP, BOOL_NOT_NAME, native_bool_not, "not", &LOCATION);


fn native_bool_and(args: Vec<Value>) -> CResult {
    if args.len() != 2 {
        return Err(CError::ArgsNotMatching(1, args.len()));
    }
    let a = args.get(0).unwrap();
    let b = args.get(1).unwrap();
    let a = if let Value::Bool(a) = a {
        *a
    } else {
        return Err(CError::TypeError((), a.clone()));
    };
    if let Value::Bool(b) = b {
        Ok(Value::Bool(a && *b))
    } else {
        Err(CError::TypeError((), b.clone()))
    }
}

impl_wrap!(BOOL_AND_WRAP, BOOL_AND_NAME, native_bool_and, "raw-and", &LOCATION);


fn native_bool_or(args: Vec<Value>) -> CResult {
    if args.len() != 2 {
        return Err(CError::ArgsNotMatching(1, args.len()));
    }
    let a = args.get(0).unwrap();
    let b = args.get(1).unwrap();
    let a = if let Value::Bool(b) = a {
        *b
    } else {
        return Err(CError::TypeError((), a.clone()));
    };
    if let Value::Bool(b) = b {
        Ok(Value::Bool(a || *b))
    } else {
        Err(CError::TypeError((), b.clone()))
    }
}

impl_wrap!(BOOL_OR_WRAP, BOOL_OR_NAME, native_bool_or, "raw-or", &LOCATION);
