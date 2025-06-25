#![feature(array_chunks)]

pub mod deserialize;
pub mod serialize;
pub use deserialize::CairoDeserialize;
pub use serialize::CairoSerialize;
