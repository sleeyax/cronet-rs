#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod buffer;

pub use buffer::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_cronet_buffer() {
        let size = 1;
        let buffer = super::Buffer::new();
        buffer.initWithAlloc(size);
        assert!(buffer.size() == size);
        buffer.destroy();
    }
}
