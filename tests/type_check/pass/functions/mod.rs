use crate::test_type;
use pijama::ty::Ty;

// Non-recursive functions
test_type!(fn_from_int_to_int, Ok(Ty::Arrow(box Ty::Int, box Ty::Int)));