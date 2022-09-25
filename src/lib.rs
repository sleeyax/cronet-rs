#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn can_create_cronet_buffer() {
        unsafe {
            let buffer = Cronet_Buffer_Create();
            Cronet_Buffer_Destroy(buffer);
        }
    }
}
