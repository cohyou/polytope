pub mod web;
pub mod test;
mod embedded;
// mod algebraic_property_graph;

mod byte;
mod valtype;
mod resulttype;
mod functype;

mod module;
mod context;

mod util;
// use algebraic_property_graph::*;

pub use embedded::make_module;
pub use embedded::decode_wasm;

pub use web::kick;
