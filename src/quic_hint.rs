use std::ffi::{CStr, CString};

use crate::{
    Cronet_QuicHintPtr, Cronet_QuicHint_Create, Cronet_QuicHint_Destroy,
    Cronet_QuicHint_alternate_port_get, Cronet_QuicHint_alternate_port_set,
    Cronet_QuicHint_host_get, Cronet_QuicHint_host_set, Cronet_QuicHint_port_get,
    Cronet_QuicHint_port_set, Destroy,
};

pub struct QuicHint {
    pub(crate) ptr: Cronet_QuicHintPtr,
}

impl QuicHint {
    pub fn new() -> Self {
        unsafe {
            QuicHint {
                ptr: Cronet_QuicHint_Create(),
            }
        }
    }

    /// Set the name of the host that supports QUIC.
    pub fn set_host(&self, host: &str) {
        unsafe {
            let c_str = CString::new(host).unwrap();
            Cronet_QuicHint_host_set(self.ptr, c_str.as_ptr());
        }
    }

    pub fn host(&self) -> &str {
        unsafe {
            let c_str = Cronet_QuicHint_host_get(self.ptr);
            let host = CStr::from_ptr(c_str).to_str().unwrap();
            host
        }
    }

    /// Set the port of the server that supports QUIC.
    pub fn set_port(&self, port: i32) {
        unsafe {
            Cronet_QuicHint_port_set(self.ptr, port);
        }
    }

    pub fn port(&self) -> i32 {
        unsafe { Cronet_QuicHint_port_get(self.ptr) }
    }

    /// Set an alternative port to use for QUIC.
    pub fn set_alternate_port(&self, port: i32) {
        unsafe {
            Cronet_QuicHint_alternate_port_set(self.ptr, port);
        }
    }

    pub fn alternate_port(&self) -> i32 {
        unsafe { Cronet_QuicHint_alternate_port_get(self.ptr) }
    }
}

impl Destroy for QuicHint {
    fn destroy(&self) {
        unsafe { Cronet_QuicHint_Destroy(self.ptr) }
    }
}

impl Default for QuicHint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Destroy;

    #[test]
    fn test_quic_hint() {
        let quic_hint = super::QuicHint::new();
        quic_hint.set_host("www.example.com");
        quic_hint.set_port(443);
        quic_hint.set_alternate_port(8443);
        assert_eq!(quic_hint.host(), "www.example.com");
        assert_eq!(quic_hint.port(), 443);
        assert_eq!(quic_hint.alternate_port(), 8443);
        quic_hint.destroy();
    }
}
