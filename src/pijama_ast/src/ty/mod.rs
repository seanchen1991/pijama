//! Types and functions related to Pijama's type system.
//!
//! The entry point for this module is the `ty_check` function which takes care of type inference
//! and checking.
// pub mod ty_error;

use std::fmt;

use thiserror::Error;

use crate::Name;
use crate::{Located, Location};

/// The type of a term.
///
/// Each variant here represents the type a term might have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    /// The type of booleans.
    Bool,
    /// The type of (signed) integers.
    Int,
    /// The [unit type](https://en.wikipedia.org/wiki/Unit_type).
    Unit,
    /// The type of functions between two types.
    Arrow(Box<Ty>, Box<Ty>),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Ty::*;
        match self {
            Bool => write!(f, "Bool"),
            Int => write!(f, "Int"),
            Unit => write!(f, "Unit"),
            Arrow(t1, t2) => {
                if let Arrow(_, _) = t1.as_ref() {
                    write!(f, "({}) -> {}", t1, t2)
                } else {
                    write!(f, "{} -> {}", t1, t2)
                }
            }
        }
    }
}

/// A type binding.
///
/// This represents a binding of a `Name` to a type and is used inside the type checker as the
/// default way of encoding that a variable has a type in the current scope.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Binding<'a> {
    pub name: Name<'a>,
    pub ty: Ty,
}

/// The type returned by methods and functions in this module.
pub type TyResult<T = Ty> = Result<T, TyError>;

/// A typing error.
///
/// Each variant here represents a reason why the type-checker could fail.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum TyError {
    /// Variant used when two types that should be equal are not.
    #[error("Unexpected type: expected {expected}, found {found}")]
    Unexpected { expected: Ty, found: Located<Ty> },
    /// Variant used when a name has not been binded to any type in the current scope.
    #[error("Name {0} is not bounded")]
    Unbound(Located<String>),
    /// Variant used when a type was expected to be a `Ty::Arrow` function type.
    #[error("Unexpected type: expected function, found {0}")]
    ExpectedFn(Located<Ty>),
    /// Variant used when a type was expected to not be a `Ty::Arrow` function type.
    #[error("Unexpected type: expected a basic type, found {0}")]
    ExpectedBasic(Located<Ty>),
    /// Variant used when a required type annotation is missing.
    #[error("Missing type: type cannot be inferred")]
    Missing(Located<()>),
}

impl TyError {
    /// Returns the location of the error.
    pub fn loc(&self) -> Location {
        match self {
            TyError::Unexpected { found, .. } => found.loc,
            TyError::Unbound(name) => name.loc,
            TyError::ExpectedBasic(ty) | TyError::ExpectedFn(ty) => ty.loc,
            TyError::Missing(unit) => unit.loc,
        }
    }
}