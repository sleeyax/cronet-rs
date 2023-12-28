use std::ffi::{CStr, CString};

use crate::{
    Cronet_EngineParamsPtr, Cronet_EngineParams_Create, Cronet_EngineParams_Destroy,
    Cronet_EngineParams_accept_language_get, Cronet_EngineParams_accept_language_set,
    Cronet_EngineParams_enable_brotli_get, Cronet_EngineParams_enable_brotli_set,
    Cronet_EngineParams_enable_check_result_get, Cronet_EngineParams_enable_check_result_set,
    Cronet_EngineParams_enable_http2_get, Cronet_EngineParams_enable_http2_set,
    Cronet_EngineParams_enable_public_key_pinning_bypass_for_local_trust_anchors_get,
    Cronet_EngineParams_enable_public_key_pinning_bypass_for_local_trust_anchors_set,
    Cronet_EngineParams_enable_quic_get, Cronet_EngineParams_enable_quic_set,
    Cronet_EngineParams_experimental_options_get, Cronet_EngineParams_experimental_options_set,
    Cronet_EngineParams_http_cache_max_size_get, Cronet_EngineParams_http_cache_max_size_set,
    Cronet_EngineParams_http_cache_mode_set, Cronet_EngineParams_network_thread_priority_get,
    Cronet_EngineParams_network_thread_priority_set, Cronet_EngineParams_public_key_pins_add,
    Cronet_EngineParams_public_key_pins_at, Cronet_EngineParams_public_key_pins_clear,
    Cronet_EngineParams_public_key_pins_size, Cronet_EngineParams_quic_hints_add,
    Cronet_EngineParams_quic_hints_at, Cronet_EngineParams_quic_hints_clear,
    Cronet_EngineParams_quic_hints_size, Cronet_EngineParams_storage_path_get,
    Cronet_EngineParams_storage_path_set, Cronet_EngineParams_user_agent_get,
    Cronet_EngineParams_user_agent_set, Destroy, PublicKeyPins, QuicHint,
};

/// Encapsulates configuration parameters for the [Engine].
pub struct EngineParams {
    pub(crate) ptr: Cronet_EngineParamsPtr,
}

impl EngineParams {
    pub fn new() -> Self {
        unsafe {
            EngineParams {
                ptr: Cronet_EngineParams_Create(),
            }
        }
    }

    /// Override strict result checking for all operations that return `RESULT`.
    /// If set to `true`, then a failed result will cause a native crash via `SIGABRT`.
    pub fn set_enable_check_result(&self, enable: bool) {
        unsafe {
            Cronet_EngineParams_enable_check_result_set(self.ptr, enable);
        }
    }

    /// Whether strict result checking is enabled.
    pub fn enable_check_result(&self) -> bool {
        unsafe { Cronet_EngineParams_enable_check_result_get(self.ptr) }
    }

    /// Set the default `User-Agent` header for all requests.
    pub fn set_user_agent(&self, user_agent: &str) {
        unsafe {
            let c_str = CString::new(user_agent).unwrap();
            Cronet_EngineParams_user_agent_set(self.ptr, c_str.as_ptr());
        }
    }

    /// Returns the `User-Agent` header value.
    pub fn user_agent(&self) -> &str {
        unsafe {
            let c_str = Cronet_EngineParams_user_agent_get(self.ptr);
            let c_str = CStr::from_ptr(c_str);
            let str_slice = c_str.to_str().unwrap();
            str_slice
        }
    }

    /// Set the default `Accept-Language` header value for all requests.
    pub fn set_accept_language(&self, accent_language: &str) {
        unsafe {
            let c_str = CString::new(accent_language).unwrap();
            Cronet_EngineParams_accept_language_set(self.ptr, c_str.as_ptr());
        }
    }

    /// Returns the `Accept-Language` header value.
    pub fn accept_language(&self) -> &str {
        unsafe {
            let c_str = Cronet_EngineParams_accept_language_get(self.ptr);
            let c_str = CStr::from_ptr(c_str);
            let str_slice = c_str.to_str().unwrap();
            str_slice
        }
    }

    /// Set the directory for HTTP Cache and Prefs Storage.
    /// The path must exist.
    pub fn set_storage_path(&self, storage_path: &str) {
        unsafe {
            let c_str = CString::new(storage_path).unwrap();
            Cronet_EngineParams_storage_path_set(self.ptr, c_str.as_ptr());
        }
    }

    /// Returns the directory for HTTP Cache and Prefs Storage.
    pub fn storage_path(&self) -> &str {
        unsafe {
            let c_str = Cronet_EngineParams_storage_path_get(self.ptr);
            let c_str = CStr::from_ptr(c_str);
            let str_slice = c_str.to_str().unwrap();
            str_slice
        }
    }

    /// Whether the <a href="https://www.chromium.org/quic">QUIC</a> protocol should be enabled.
    /// If QUIC is enabled, the QUIC User Agent Id containing application name and Cronet version is sent to the destination.
    pub fn set_enable_quic(&self, enable: bool) {
        unsafe {
            Cronet_EngineParams_enable_quic_set(self.ptr, enable);
        }
    }

    /// Returns whether the <a href="https://www.chromium.org/quic">QUIC</a> protocol is enabled.
    pub fn enable_quic(&self) -> bool {
        unsafe { Cronet_EngineParams_enable_quic_get(self.ptr) }
    }

    /// Whether the <a href="https://tools.ietf.org/html/rfc7540">HTTP/2</a> protocol should be enabled.
    pub fn set_enable_http_2(&self, enable: bool) {
        unsafe {
            Cronet_EngineParams_enable_http2_set(self.ptr, enable);
        }
    }

    /// Whether the <a href="https://tools.ietf.org/html/rfc7540">HTTP/2</a> protocol is enabled.
    pub fn enable_http_2(&self) -> bool {
        unsafe { Cronet_EngineParams_enable_http2_get(self.ptr) }
    }

    /// Whether <a href="https://tools.ietf.org/html/rfc7932">Brotli</a> compression should be enabled.
    pub fn set_enable_brotli(&self, enable: bool) {
        unsafe {
            Cronet_EngineParams_enable_brotli_set(self.ptr, enable);
        }
    }

    /// Whether <a href="https://tools.ietf.org/html/rfc7932">Brotli</a> compression is enabled.
    pub fn enable_brotli(&self) -> bool {
        unsafe { Cronet_EngineParams_enable_brotli_get(self.ptr) }
    }

    /// Enable or disable caching of HTTP data and other information like QUIC.
    pub fn set_http_cache_mode(&self, mode: HttpCacheMode) {
        unsafe {
            Cronet_EngineParams_http_cache_mode_set(self.ptr, mode as u32);
        }
    }

    /// Returns the current HTTP cache mode.
    pub fn http_cache_mode(&self) -> HttpCacheMode {
        unsafe {
            Cronet_EngineParams_http_cache_max_size_get(self.ptr)
                .try_into()
                .unwrap()
        }
    }

    /// Set the maximum size of the HTTP cache in bytes.
    /// The value set here is advisory and may be exceeded at times.
    pub fn set_http_cache_max_size(&self, max_size: i64) {
        unsafe {
            Cronet_EngineParams_http_cache_max_size_set(self.ptr, max_size);
        }
    }

    /// Returns the maximum size of the HTTP cache in bytes.
    pub fn http_cache_max_size(&self) -> i64 {
        unsafe { Cronet_EngineParams_http_cache_max_size_get(self.ptr) }
    }

    pub fn add_quic_hint(&self, quic_hint: QuicHint) {
        unsafe {
            Cronet_EngineParams_quic_hints_add(self.ptr, quic_hint.ptr);
        }
    }

    pub fn quic_hints_size(&self) -> u32 {
        unsafe { Cronet_EngineParams_quic_hints_size(self.ptr) }
    }

    pub fn quic_hint_at(&self, index: u32) -> QuicHint {
        unsafe {
            let ptr = Cronet_EngineParams_quic_hints_at(self.ptr, index);
            QuicHint { ptr }
        }
    }

    pub fn clear_quic_hints(&self) {
        unsafe {
            Cronet_EngineParams_quic_hints_clear(self.ptr);
        }
    }

    pub fn add_public_key_pins(&self, public_key_pins: PublicKeyPins) {
        unsafe {
            Cronet_EngineParams_public_key_pins_add(self.ptr, public_key_pins.ptr);
        }
    }

    pub fn public_key_pins_size(&self) -> u32 {
        unsafe { Cronet_EngineParams_public_key_pins_size(self.ptr) }
    }

    pub fn public_key_pins_at(&self, index: u32) -> PublicKeyPins {
        unsafe {
            let ptr = Cronet_EngineParams_public_key_pins_at(self.ptr, index);
            PublicKeyPins { ptr }
        }
    }

    pub fn clear_public_key_pins(&self) {
        unsafe {
            Cronet_EngineParams_public_key_pins_clear(self.ptr);
        }
    }

    /// Enable or disable public key pinning bypass for local trust anchors.
    /// Disabling it for local trust anchors is highly discouraged since it may prohibit the app from communicating with the pinned hosts.
    /// E.g., a user may want to send all traffic through an SSL enabled proxy by changing the device proxy settings
    /// and adding the proxy certificate to the list of local trust anchor.
    /// Disabling it will most likely prevent the app from sending any traffic to the pinned hosts.
    /// For more information see 'How does key pinning interact with local proxies and filters?' at https://www.chromium.org/Home/chromium-security/security-faq
    pub fn set_enable_public_key_pinning_bypass_for_local_trust_anchors(&self, enable: bool) {
        unsafe {
            Cronet_EngineParams_enable_public_key_pinning_bypass_for_local_trust_anchors_set(
                self.ptr, enable,
            );
        }
    }

    pub fn enable_public_key_pinning_bypass_for_local_trust_anchors(&self) -> bool {
        unsafe {
            Cronet_EngineParams_enable_public_key_pinning_bypass_for_local_trust_anchors_get(
                self.ptr,
            )
        }
    }

    // Set the network thread priority.
    // On Android, corresponds to android.os.Process.setThreadPriority() values.
    // On iOS, corresponds to NSThread::setThreadPriority values.
    // It's recommended to leave this unspecified on other platforms.
    pub fn set_network_thread_priority(&self, priority: f64) {
        unsafe { Cronet_EngineParams_network_thread_priority_set(self.ptr, priority) }
    }

    pub fn network_thread_priority(&self) -> f64 {
        unsafe { Cronet_EngineParams_network_thread_priority_get(self.ptr) }
    }

    /// Specify JSON formatted string of experimental options to be used in Cronet Engine.
    pub fn set_experimental_options(&self, options: &str) {
        unsafe {
            let c_str = CString::new(options).unwrap();
            Cronet_EngineParams_experimental_options_set(self.ptr, c_str.as_ptr());
        }
    }

    pub fn experimental_options(&self) -> &str {
        unsafe {
            let c_str = Cronet_EngineParams_experimental_options_get(self.ptr);
            let c_str = CStr::from_ptr(c_str);
            let str_slice = c_str.to_str().unwrap();
            str_slice
        }
    }
}

impl Destroy for EngineParams {
    fn destroy(&self) {
        unsafe { Cronet_EngineParams_Destroy(self.ptr) }
    }
}

impl Default for EngineParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpCacheMode {
    /// Disable HTTP cache.
    /// Some data may still be temporarily stored in memory.
    Disabled = 0,

    /// Enable in-memory HTTP cache, including HTTP data.
    InMemory = 1,

    /// Enable on-disk cache, excluding HTTP data.
    /// Requires a valid storage path (see`EngineParams.set_storage_path`).
    DiskNoHttp = 2,

    /// Enable on-disk cache, including HTTP data.
    /// Requires a valid storage path (see `EngineParams.set_storage_path`).
    Disk = 3,
}

impl TryFrom<i64> for HttpCacheMode {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HttpCacheMode::Disabled),
            1 => Ok(HttpCacheMode::InMemory),
            2 => Ok(HttpCacheMode::DiskNoHttp),
            3 => Ok(HttpCacheMode::Disk),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Destroy;

    #[test]
    fn it_enables_check_result() {
        let engine_params = super::EngineParams::new();
        engine_params.set_enable_check_result(true);
        assert_eq!(engine_params.enable_check_result(), true);
        engine_params.destroy();
    }

    #[test]
    fn it_sets_user_agent() {
        let engine_params = super::EngineParams::new();
        engine_params.set_user_agent("test");
        assert_eq!(engine_params.user_agent(), "test");
        engine_params.destroy();
    }

    #[test]
    fn it_sets_accept_language() {
        let engine_params = super::EngineParams::new();
        engine_params.set_accept_language("test");
        assert_eq!(engine_params.accept_language(), "test");
        engine_params.destroy();
    }

    #[test]
    fn it_sets_storage_path() {
        let engine_params = super::EngineParams::new();
        engine_params.set_storage_path("test");
        assert_eq!(engine_params.storage_path(), "test");
        engine_params.destroy();
    }

    #[test]
    fn it_enables_quic() {
        let engine_params = super::EngineParams::new();
        engine_params.set_enable_quic(true);
        assert_eq!(engine_params.enable_quic(), true);
        engine_params.destroy();
    }

    #[test]
    fn it_enables_http_2() {
        let engine_params = super::EngineParams::new();
        engine_params.set_enable_http_2(true);
        assert_eq!(engine_params.enable_http_2(), true);
        engine_params.destroy();
    }

    #[test]
    fn it_enables_brotli() {
        let engine_params = super::EngineParams::new();
        engine_params.set_enable_brotli(true);
        assert_eq!(engine_params.enable_brotli(), true);
        engine_params.destroy();
    }

    #[test]
    fn it_sets_http_cache_mode() {
        let engine_params = super::EngineParams::new();
        engine_params.set_http_cache_mode(super::HttpCacheMode::Disabled);
        assert_eq!(
            engine_params.http_cache_mode(),
            super::HttpCacheMode::Disabled
        );
        engine_params.destroy();
    }

    #[test]
    fn it_sets_http_cache_max_size() {
        let engine_params = super::EngineParams::new();
        engine_params.set_http_cache_max_size(10);
        assert_eq!(engine_params.http_cache_max_size(), 10);
        engine_params.destroy();
    }

    #[test]
    fn it_sets_experimental_options() {
        let engine_params = super::EngineParams::new();
        engine_params.set_experimental_options("test");
        assert_eq!(engine_params.experimental_options(), "test");
        engine_params.destroy();
    }

    #[test]
    fn it_sets_network_thread_priority() {
        let engine_params = super::EngineParams::new();
        engine_params.set_network_thread_priority(10.0);
        assert_eq!(engine_params.network_thread_priority(), 10.0);
        engine_params.destroy();
    }

    #[test]
    fn it_enables_public_key_pinning_bypass_for_local_trust_anchors() {
        let engine_params = super::EngineParams::new();
        engine_params.set_enable_public_key_pinning_bypass_for_local_trust_anchors(true);
        assert_eq!(
            engine_params.enable_public_key_pinning_bypass_for_local_trust_anchors(),
            true
        );
        engine_params.destroy();
    }

    #[test]
    fn test_quic_hints() {
        let engine_params = super::EngineParams::new();
        let quic_hint = super::QuicHint::new();
        quic_hint.set_host("www.example.com");
        engine_params.add_quic_hint(quic_hint);
        assert_eq!(engine_params.quic_hints_size(), 1);
        let quic_hint2 = engine_params.quic_hint_at(0);
        assert_eq!(quic_hint2.host(), "www.example.com");
        engine_params.clear_quic_hints();
        assert_eq!(engine_params.quic_hints_size(), 0);
        engine_params.destroy();
    }

    #[test]
    fn test_public_key_pins() {
        let engine_params = super::EngineParams::new();
        let public_key_pins = super::PublicKeyPins::new();
        public_key_pins.add("sha256/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
        engine_params.add_public_key_pins(public_key_pins);
        assert_eq!(engine_params.public_key_pins_size(), 1);
        let public_key_pins2 = engine_params.public_key_pins_at(0);
        assert_eq!(
            public_key_pins2.at(0),
            "sha256/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
        );
        engine_params.clear_public_key_pins();
        assert_eq!(engine_params.public_key_pins_size(), 0);
        engine_params.destroy();
    }
}
