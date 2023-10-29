use std::{
    ffi::{CStr, CString},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    Cronet_PublicKeyPinsPtr, Cronet_PublicKeyPins_Create, Cronet_PublicKeyPins_Destroy,
    Cronet_PublicKeyPins_expiration_date_get, Cronet_PublicKeyPins_expiration_date_set,
    Cronet_PublicKeyPins_host_get, Cronet_PublicKeyPins_host_set,
    Cronet_PublicKeyPins_include_subdomains_get, Cronet_PublicKeyPins_include_subdomains_set,
    Cronet_PublicKeyPins_pins_sha256_add, Cronet_PublicKeyPins_pins_sha256_at,
    Cronet_PublicKeyPins_pins_sha256_clear, Cronet_PublicKeyPins_pins_sha256_size, Destroy,
};

pub struct PublicKeyPins {
    pub(crate) ptr: Cronet_PublicKeyPinsPtr,
}

impl PublicKeyPins {
    pub fn new() -> Self {
        unsafe {
            PublicKeyPins {
                ptr: Cronet_PublicKeyPins_Create(),
            }
        }
    }

    /// Set the name of the host to which the public keys should be pinned.
    pub fn set_host(&self, host: &str) {
        unsafe {
            let c_str = CString::new(host).unwrap();
            Cronet_PublicKeyPins_host_set(self.ptr, c_str.as_ptr());
        }
    }

    pub fn host(&self) -> String {
        unsafe {
            let c_str = Cronet_PublicKeyPins_host_get(self.ptr);
            let host = CStr::from_ptr(c_str).to_string_lossy().into_owned();
            host
        }
    }

    /// Add pins.
    /// Each pin is the SHA-256 cryptographic hash (in the form of "sha256/<base64-hash-value>") of the DER-encoded ASN.1
    /// representation of the Subject Public Key Info (SPKI) of the host's X.509 certificate.
    /// Although the method does not mandate the presence of the backup pin
    /// that can be used if the control of the primary private key has been
    /// lost, it is highly recommended to supply one.
    pub fn add(&self, sha256: &str) {
        unsafe {
            let c_str = CString::new(sha256).unwrap();
            Cronet_PublicKeyPins_pins_sha256_add(self.ptr, c_str.as_ptr());
        }
    }

    pub fn size(&self) -> u32 {
        unsafe { Cronet_PublicKeyPins_pins_sha256_size(self.ptr) }
    }

    pub fn at(&self, index: u32) -> String {
        unsafe {
            let c_str = Cronet_PublicKeyPins_pins_sha256_at(self.ptr, index);
            CStr::from_ptr(c_str).to_string_lossy().into_owned()
        }
    }

    pub fn clear(&self) {
        unsafe {
            Cronet_PublicKeyPins_pins_sha256_clear(self.ptr);
        }
    }

    pub fn set_include_subdomains(&self, enable: bool) {
        unsafe {
            Cronet_PublicKeyPins_include_subdomains_set(self.ptr, enable);
        }
    }

    pub fn include_subdomains(&self) -> bool {
        unsafe { Cronet_PublicKeyPins_include_subdomains_get(self.ptr) }
    }

    /// Set the expiration date for the pins in milliseconds since epoch.
    pub fn set_expiration_date_millis(&self, date: i64) {
        unsafe {
            Cronet_PublicKeyPins_expiration_date_set(self.ptr, date);
        }
    }

    pub fn expiration_date_millis(&self) -> i64 {
        unsafe { Cronet_PublicKeyPins_expiration_date_get(self.ptr) }
    }

    pub fn set_expiration_date(&self, time: SystemTime) {
        let duration = time.duration_since(UNIX_EPOCH).unwrap();
        let milliseconds = duration.as_millis();
        self.set_expiration_date_millis(milliseconds as i64);
    }

    pub fn expiration_date(&self) -> SystemTime {
        let milliseconds = self.expiration_date_millis();
        let duration = Duration::from_millis(milliseconds as u64);
        UNIX_EPOCH + duration
    }
}

impl Destroy for PublicKeyPins {
    fn destroy(&self) {
        unsafe { Cronet_PublicKeyPins_Destroy(self.ptr) }
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use crate::Destroy;

    #[test]
    fn test_public_key_pins() {
        let public_key_pins = super::PublicKeyPins::new();
        let now = std::time::SystemTime::now();
        let now_millis = now.duration_since(UNIX_EPOCH).unwrap().as_millis();
        public_key_pins.set_expiration_date_millis(now_millis as i64);
        assert_eq!(public_key_pins.expiration_date_millis(), now_millis as i64);
        public_key_pins.set_host("www.example.com");
        assert_eq!(public_key_pins.host(), "www.example.com");
        public_key_pins.add("sha256/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
        assert_eq!(
            public_key_pins.at(0),
            "sha256/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
        );
        public_key_pins.set_include_subdomains(true);
        assert_eq!(public_key_pins.include_subdomains(), true);
        public_key_pins.clear();
        assert_eq!(public_key_pins.size(), 0);
        public_key_pins.destroy()
    }
}
