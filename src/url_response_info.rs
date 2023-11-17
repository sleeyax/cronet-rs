use std::ffi::{CStr, CString};

use http::{HeaderValue, Response, StatusCode, Version};

use crate::{
    Cronet_UrlResponseInfoPtr, Cronet_UrlResponseInfo_Create, Cronet_UrlResponseInfo_Destroy,
    Cronet_UrlResponseInfo_all_headers_list_add, Cronet_UrlResponseInfo_all_headers_list_at,
    Cronet_UrlResponseInfo_all_headers_list_clear, Cronet_UrlResponseInfo_all_headers_list_size,
    Cronet_UrlResponseInfo_http_status_code_get, Cronet_UrlResponseInfo_http_status_code_set,
    Cronet_UrlResponseInfo_http_status_text_get, Cronet_UrlResponseInfo_http_status_text_set,
    Cronet_UrlResponseInfo_negotiated_protocol_get, Cronet_UrlResponseInfo_negotiated_protocol_set,
    Cronet_UrlResponseInfo_proxy_server_get, Cronet_UrlResponseInfo_proxy_server_set,
    Cronet_UrlResponseInfo_received_byte_count_get, Cronet_UrlResponseInfo_received_byte_count_set,
    Cronet_UrlResponseInfo_url_chain_add, Cronet_UrlResponseInfo_url_chain_at,
    Cronet_UrlResponseInfo_url_chain_clear, Cronet_UrlResponseInfo_url_chain_size,
    Cronet_UrlResponseInfo_url_get, Cronet_UrlResponseInfo_url_set,
    Cronet_UrlResponseInfo_was_cached_get, Cronet_UrlResponseInfo_was_cached_set, Destroy,
    HttpHeader,
};

pub struct UrlResponseInfo {
    pub(crate) ptr: Cronet_UrlResponseInfoPtr,
}

impl UrlResponseInfo {
    pub fn new() -> Self {
        unsafe {
            UrlResponseInfo {
                ptr: Cronet_UrlResponseInfo_Create(),
            }
        }
    }

    /// The URL the response is for.
    /// This is the URL after following redirects, so it may not be the originally requested URL
    pub fn url(&self) -> &str {
        unsafe {
            let url = Cronet_UrlResponseInfo_url_get(self.ptr);
            let url = CStr::from_ptr(url);
            url.to_str().unwrap()
        }
    }

    pub fn set_url(&self, url: &str) {
        unsafe {
            let c_string = CString::new(url).unwrap();
            Cronet_UrlResponseInfo_url_set(self.ptr, c_string.as_ptr());
        }
    }

    /// The URL chain size.
    /// The first entry is the originally requested URL; the following entries are redirects followed.
    pub fn url_chain_size(&self) -> u32 {
        unsafe { Cronet_UrlResponseInfo_url_chain_size(self.ptr) }
    }

    /// The URL at the given index in the chain.
    /// The first entry is the originally requested URL; the following entries are redirects followed.
    pub fn url_chain_at(&self, index: u32) -> &str {
        unsafe {
            let url = Cronet_UrlResponseInfo_url_chain_at(self.ptr, index);
            let url = CStr::from_ptr(url);
            url.to_str().unwrap()
        }
    }

    /// Add a URL to the chain.
    /// The first entry is the originally requested URL; the following entries are redirects followed.
    pub fn add_url_chain(&self, url: &str) {
        unsafe {
            let c_string = CString::new(url).unwrap();
            Cronet_UrlResponseInfo_url_chain_add(self.ptr, c_string.as_ptr());
        }
    }

    /// Clears the URL chain.
    /// The first entry is the originally requested URL; the following entries are redirects followed.
    pub fn clear_url_chain(&self) {
        unsafe {
            Cronet_UrlResponseInfo_url_chain_clear(self.ptr);
        }
    }

    /// The HTTP status code.
    /// When a resource is retrieved from the cache, whether it was revalidated or not, the original status code is returned.
    pub fn status_code(&self) -> i32 {
        unsafe { Cronet_UrlResponseInfo_http_status_code_get(self.ptr) }
    }

    pub fn set_status_code(&self, code: i32) {
        unsafe { Cronet_UrlResponseInfo_http_status_code_set(self.ptr, code) }
    }

    /// The HTTP status text of the status line.
    /// For example, if the request received a "HTTP/1.1 200 OK" response, this method returns "OK".
    pub fn status_text(&self) -> &str {
        unsafe {
            let text = Cronet_UrlResponseInfo_http_status_text_get(self.ptr);
            let text = CStr::from_ptr(text);
            text.to_str().unwrap()
        }
    }

    pub fn set_status_text(&self, text: &str) {
        unsafe {
            let c_string = CString::new(text).unwrap();
            Cronet_UrlResponseInfo_http_status_text_set(self.ptr, c_string.as_ptr());
        }
    }

    pub fn header_size(&self) -> u32 {
        unsafe {
            let size = Cronet_UrlResponseInfo_all_headers_list_size(self.ptr);
            size
        }
    }

    pub fn header_at(&self, index: u32) -> HttpHeader {
        unsafe {
            let ptr = Cronet_UrlResponseInfo_all_headers_list_at(self.ptr, index);
            HttpHeader { ptr }
        }
    }

    pub fn add_header(&self, header: HttpHeader) {
        unsafe {
            Cronet_UrlResponseInfo_all_headers_list_add(self.ptr, header.ptr);
        }
    }

    pub fn clear_headers(&self) {
        unsafe {
            Cronet_UrlResponseInfo_all_headers_list_clear(self.ptr);
        }
    }

    /// Set to `true` if the response came from the cache, including
    /// requests that were revalidated over the network before being retrieved
    /// from the cache, `false` otherwise.
    pub fn cached(&self) -> bool {
        unsafe { Cronet_UrlResponseInfo_was_cached_get(self.ptr) }
    }

    pub fn set_cached(&self, cached: bool) {
        unsafe { Cronet_UrlResponseInfo_was_cached_set(self.ptr, cached) }
    }

    /// The protocol (for example 'quic/1+spdy/3') negotiated with the server.
    /// An empty string if no protocol was negotiated, the protocol is
    /// not known, or when using plain HTTP or HTTPS.
    pub fn negotiated_protocol(&self) -> &'static str {
        unsafe {
            let protocol = Cronet_UrlResponseInfo_negotiated_protocol_get(self.ptr);
            let protocol = CStr::from_ptr(protocol);
            let protocol = protocol.to_str().unwrap();
            protocol
        }
    }

    pub fn set_negotiated_protocol(&self, protocol: &str) {
        unsafe {
            let c_string = CString::new(protocol).unwrap();
            Cronet_UrlResponseInfo_negotiated_protocol_set(self.ptr, c_string.as_ptr());
        }
    }

    /// The proxy server that was used for the request.
    pub fn proxy_server(&self) -> &'static str {
        unsafe {
            let server = Cronet_UrlResponseInfo_proxy_server_get(self.ptr);
            let server = CStr::from_ptr(server).to_str().unwrap();
            server
        }
    }

    pub fn set_proxy_server(&self, proxy: &str) {
        unsafe {
            let c_string = CString::new(proxy).unwrap();
            Cronet_UrlResponseInfo_proxy_server_set(self.ptr, c_string.as_ptr());
        }
    }

    /// The amount of bytes received from the network to process this request.
    /// This count may ignore certain overheads (for example IP and TCP/UDP framing, SSL handshake and framing, proxy handling).
    /// This count is taken prior to decompression (for example GZIP and Brotli) and includes headers and data from all redirects.
    pub fn received_byte_count(&self) -> i64 {
        unsafe { Cronet_UrlResponseInfo_received_byte_count_get(self.ptr) }
    }

    pub fn set_received_byte_count(&self, count: i64) {
        unsafe { Cronet_UrlResponseInfo_received_byte_count_set(self.ptr, count) }
    }
}

impl Destroy for UrlResponseInfo {
    fn destroy(&self) {
        unsafe {
            Cronet_UrlResponseInfo_Destroy(self.ptr);
        }
    }
}

#[cfg(feature = "client")]
impl<T> Into<Response<T>> for UrlResponseInfo
where
    T: Default,
{
    fn into(self) -> Response<T> {
        let mut response = Response::default();

        // Set HTTP version
        let version = match self.negotiated_protocol() {
            "http/0.9" => Version::HTTP_09,
            "http/1.0" => Version::HTTP_10,
            "http/1.1" => Version::HTTP_11,
            "h2" => Version::HTTP_2,
            "h3" => Version::HTTP_3,
            "" => Version::HTTP_11,
            version => panic!("Server responded with unknown HTTP version '{}'", version),
        };
        *response.version_mut() = version;

        // Set status code
        let status_code = self.status_code();
        *response.status_mut() = StatusCode::from_u16(status_code as u16).unwrap();

        // Set headers
        let header_size = self.header_size();
        for i in 0..header_size {
            let header = self.header_at(i);
            let name = header.name();
            let value = header.value();
            response
                .headers_mut()
                .insert(name, HeaderValue::from_static(value));
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use crate::Destroy;

    #[test]
    fn test_url_response_info() {
        let url_response_info = super::UrlResponseInfo::new();
        url_response_info.set_url("https://www.google.com");
        assert_eq!(url_response_info.url(), "https://www.google.com");
        url_response_info.set_status_code(200);
        assert_eq!(url_response_info.status_code(), 200);
        url_response_info.set_status_text("OK");
        assert_eq!(url_response_info.status_text(), "OK");
        url_response_info.set_cached(true);
        assert_eq!(url_response_info.cached(), true);
        url_response_info.set_negotiated_protocol("quic/1+spdy/3");
        assert_eq!(url_response_info.negotiated_protocol(), "quic/1+spdy/3");
        url_response_info.set_proxy_server("proxy");
        assert_eq!(url_response_info.proxy_server(), "proxy");
        url_response_info.set_received_byte_count(100);
        assert_eq!(url_response_info.received_byte_count(), 100);
        url_response_info.destroy();
    }

    #[test]
    fn it_gets_url_chain() {
        let url_response_info = super::UrlResponseInfo::new();
        url_response_info.add_url_chain("https://www.google.com");
        assert_eq!(url_response_info.url_chain_size(), 1);
        assert_eq!(url_response_info.url_chain_at(0), "https://www.google.com");
        url_response_info.clear_url_chain();
        assert_eq!(url_response_info.url_chain_size(), 0);
        url_response_info.destroy();
    }

    #[test]
    fn it_gets_headers() {
        let url_response_info = super::UrlResponseInfo::new();
        let header = crate::HttpHeader::new();
        header.set_name("name");
        header.set_value("value");
        url_response_info.add_header(header);
        assert_eq!(url_response_info.header_size(), 1);
        let header2 = url_response_info.header_at(0);
        assert_eq!(header2.name(), "name");
        assert_eq!(header2.value(), "value");
        url_response_info.clear_headers();
        assert_eq!(url_response_info.header_size(), 0);
        url_response_info.destroy();
    }
}
