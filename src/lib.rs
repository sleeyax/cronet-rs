#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod buffer;
mod buffer_callback;
mod destroy;

pub use buffer::*;
pub use buffer_callback::*;
pub use destroy::*;
