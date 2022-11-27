#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod buffer;
mod buffer_callback;

pub use buffer::*;
pub use buffer_callback::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_buffer() {
        let size = 2;
        let buffer = super::Buffer::new();
        buffer.initWithAlloc(size);
        assert_eq!(buffer.size(), size);
        buffer.destroy();
    }

    #[test]
    fn can_create_buffer_callback() {
        // TODO: find out why the callback isn't actually called...
        let callback = super::BufferCallback::new( |self_, buffer| {
            assert!(buffer.size() >  0);
        });
        callback.destroy();
    }
}
