// An helper crate that exposes internal corelib function.
// This crate is compiled with an older edition that does not enforce visibity rules.

use core::internal::bounded_int::{
    BoundedInt, add, constrain, div_rem, sub, AddHelper, ConstrainHelper, DivRemHelper, SubHelper,
};

use core::integer::upcast;
