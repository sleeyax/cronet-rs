use std::ffi::{CStr, CString};

use crate::{
    Cronet_HttpHeaderPtr, Cronet_HttpHeader_Create, Cronet_HttpHeader_Destroy,
    Cronet_HttpHeader_name_get, Cronet_HttpHeader_name_set, Cronet_HttpHeader_value_get,
    Cronet_HttpHeader_value_set, Destroy,
};

pub struct HttpHeader {
    pub(crate) ptr: Cronet_HttpHeaderPtr,
}

impl HttpHeader {
    pub fn new() -> Self {
        unsafe {
            HttpHeader {
                ptr: Cronet_HttpHeader_Create(),
            }
        }
    }

    /// Get the name of this header.
    pub fn name(&self) -> &'static str {
        unsafe {
            let c_name = Cronet_HttpHeader_name_get(self.ptr);
            let name = std::ffi::CStr::from_ptr(c_name).to_str().unwrap();
            name
        }
    }

    /// Set the name of this header.
    pub fn set_name(&self, name: &str) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            Cronet_HttpHeader_name_set(self.ptr, c_name.as_ptr());
        }
    }

    pub fn value(&self) -> &'static str {
        unsafe {
            let c_value = Cronet_HttpHeader_value_get(self.ptr);
            let value = CStr::from_ptr(c_value).to_str().unwrap();
            value
        }
    }

    pub fn set_value(&self, value: &str) {
        unsafe {
            let c_value = CString::new(value).unwrap();
            Cronet_HttpHeader_value_set(self.ptr, c_value.as_ptr());
        }
    }
}

impl Destroy for HttpHeader {
    fn destroy(&self) {
        unsafe { Cronet_HttpHeader_Destroy(self.ptr) }
    }
}

impl Default for HttpHeader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Destroy;

    #[test]
    fn test_http_header() {
        let http_header = super::HttpHeader::new();
        http_header.set_name("name");
        http_header.set_value("value");
        assert_eq!(http_header.name(), "name");
        assert_eq!(http_header.value(), "value");
        http_header.destroy();
    }
}
