use crate::{
    BufferCallback, Cronet_BufferPtr, Cronet_Buffer_Create, Cronet_Buffer_Destroy,
    Cronet_Buffer_GetData, Cronet_Buffer_GetSize, Cronet_Buffer_InitWithAlloc,
    Cronet_Buffer_InitWithDataAndCallback, Cronet_RawDataPtr, Destroy,
};

pub struct Buffer {
    pub(crate) ptr: Cronet_BufferPtr,
}

impl Buffer {
    pub fn new() -> Self {
        unsafe {
            Buffer {
                ptr: Cronet_Buffer_Create(),
            }
        }
    }

    pub fn new_with_size(size: u64) -> Self {
        let buffer = Buffer::new();
        buffer.init_size(size);
        buffer
    }

    pub fn new_with_data_and_callback<T>(
        data: Box<T>,
        size: u64,
        callback: BufferCallback,
    ) -> Self {
        let buffer = Buffer::new();
        buffer.init_data_and_callback(data, size, callback);
        buffer
    }

    pub fn init_size(&self, size: u64) {
        unsafe {
            Cronet_Buffer_InitWithAlloc(self.ptr, size);
        }
    }

    pub fn init_data_and_callback<T>(&self, data: Box<T>, size: u64, callback: BufferCallback) {
        unsafe {
            let dataPtr = Box::into_raw(data);
            Cronet_Buffer_InitWithDataAndCallback(
                self.ptr,
                dataPtr as Cronet_RawDataPtr,
                size,
                callback.ptr,
            );
        }
    }

    pub fn size(&self) -> u64 {
        unsafe { Cronet_Buffer_GetSize(self.ptr) }
    }

    pub fn data<T>(&self) -> Box<T> {
        unsafe {
            let dataPtr: Cronet_RawDataPtr = Cronet_Buffer_GetData(self.ptr);
            Box::from_raw(dataPtr as *mut T)
        }
    }
}

impl Destroy for Buffer {
    fn destroy(&self) {
        unsafe {
            Cronet_Buffer_Destroy(self.ptr);
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Destroy;

    #[test]
    fn it_creates_empty_buffer() {
        let buffer = super::Buffer::new();
        assert_eq!(buffer.size(), 0);
        buffer.destroy();
    }

    #[test]
    fn it_creates_buffer() {
        let buffer = super::Buffer::new_with_size(10);
        assert_eq!(buffer.size(), 10);
        buffer.destroy();
    }

    #[test]
    fn it_creates_buffer_with_data_and_callback() {
        // Vector buffer
        let data: [u8; 5] = [1, 2, 3, 4, 5];
        let buffer = super::Buffer::new_with_data_and_callback(
            Box::new(data),
            data.len() as u64,
            super::BufferCallback::new(|self_, buffer| {
                assert_eq!(buffer.size(), 5);
                self_.destroy(); // destroy the callback itself
            }),
        );
        assert_eq!(buffer.size(), data.len() as u64);
        assert_eq!(*buffer.data::<[u8; 5]>(), data);
        buffer.destroy();

        // String buffer
        let data = String::from("test");
        let buffer = super::Buffer::new_with_data_and_callback(
            Box::new(data.clone()),
            data.len() as u64,
            super::BufferCallback::new(|self_, buffer| {
                assert_eq!(buffer.size(), 4);
                self_.destroy(); // destroy the callback itself
            }),
        );
        assert_eq!(buffer.size(), data.len() as u64);
        assert_eq!(*buffer.data::<String>(), data);
        buffer.destroy();
    }
}
