use askama::Template;

pub mod core;
pub mod section;
pub mod web;
pub mod test;
pub mod embedded;
// mod algebraic_property_graph;

pub use section::*;
pub use web::*;
pub use test::*;
pub use embedded::*;
// pub use algebraic_property_graph::*;