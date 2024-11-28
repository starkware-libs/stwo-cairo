//! Utility crate that exports internal corelib function.
//! This crate is compiled with an older edition that does not enforce visibity rules.

use core::integer::upcast;
use core::internal::bounded_int::{
    AddHelper, BoundedInt, ConstrainHelper, DivRemHelper, SubHelper, add, constrain, div_rem, sub,
};
