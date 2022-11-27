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
    fn can_create_buffer_with_callback() {
        let buffer = super::Buffer::new();
        let data = String::from("test");
        let callback = super::BufferCallback::new( |self_, buffer| {
            println!("Buffer callback called! Buffer size: {}.", buffer.size());
            assert!(buffer.size() >  0);
            self_.destroy(); // destroys the callback itself.
        });
        buffer.initWithDataAndCallback(Box::new(&data), data.len() as u64, callback);
        buffer.destroy();
    }
}
