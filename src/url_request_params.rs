use std::ffi::{CStr, CString};

use http::{request::Parts, Request};

use crate::{
    client::{Body, BodyUploadDataProvider},
    Annotation, Cronet_UrlRequestParamsPtr, Cronet_UrlRequestParams_Create,
    Cronet_UrlRequestParams_Destroy, Cronet_UrlRequestParams_REQUEST_PRIORITY,
    Cronet_UrlRequestParams_allow_direct_executor_get,
    Cronet_UrlRequestParams_allow_direct_executor_set, Cronet_UrlRequestParams_annotations_add,
    Cronet_UrlRequestParams_annotations_at, Cronet_UrlRequestParams_annotations_clear,
    Cronet_UrlRequestParams_annotations_size, Cronet_UrlRequestParams_disable_cache_get,
    Cronet_UrlRequestParams_disable_cache_set, Cronet_UrlRequestParams_http_method_get,
    Cronet_UrlRequestParams_http_method_set, Cronet_UrlRequestParams_idempotency_get,
    Cronet_UrlRequestParams_idempotency_set, Cronet_UrlRequestParams_priority_get,
    Cronet_UrlRequestParams_priority_set, Cronet_UrlRequestParams_request_finished_executor_get,
    Cronet_UrlRequestParams_request_finished_executor_set,
    Cronet_UrlRequestParams_request_finished_listener_get,
    Cronet_UrlRequestParams_request_finished_listener_set,
    Cronet_UrlRequestParams_request_headers_add, Cronet_UrlRequestParams_request_headers_at,
    Cronet_UrlRequestParams_request_headers_clear, Cronet_UrlRequestParams_request_headers_size,
    Cronet_UrlRequestParams_upload_data_provider_executor_get,
    Cronet_UrlRequestParams_upload_data_provider_executor_set,
    Cronet_UrlRequestParams_upload_data_provider_get,
    Cronet_UrlRequestParams_upload_data_provider_set, Destroy, Executor, HttpHeader,
    RequestFinishedInfoListener, UploadDataProvider,
};

/// Parameters for [crate::UrlRequest].
/// Allows configuring requests before initializing them with `init_with_params()`.
pub struct UrlRequestParams {
    pub(crate) ptr: Cronet_UrlRequestParamsPtr,
}

impl UrlRequestParams {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: Cronet_UrlRequestParams_Create(),
            }
        }
    }

    /// Sets the HTTP method verb to use for this request.
    ///
    /// The default when this value is not set is "GET" if the request has no body or "POST" if it does.
    ///
    /// Allowed methods are `GET`, `HEAD`, `DELETE`, `POST`, `PUT` and `CONNECT`.
    pub fn set_method(&self, http_method: &str) {
        unsafe {
            let c_str = CString::new(http_method).unwrap();
            Cronet_UrlRequestParams_http_method_set(self.ptr, c_str.as_ptr());
        }
    }

    pub fn method(&self) -> String {
        unsafe {
            let c_str = Cronet_UrlRequestParams_http_method_get(self.ptr);
            let c_str = CStr::from_ptr(c_str);
            let str_slice = c_str.to_str().unwrap();
            str_slice.to_owned()
        }
    }

    /// Add an HTTP header to this request.
    /// Note that the header you provide is destroyed after it has been added to the request.
    pub fn add_header(&self, header: HttpHeader) {
        unsafe {
            Cronet_UrlRequestParams_request_headers_add(self.ptr, header.ptr);
            header.destroy();
        }
    }

    pub fn header_size(&self) -> u32 {
        unsafe { Cronet_UrlRequestParams_request_headers_size(self.ptr) }
    }

    pub fn header_at(&self, index: u32) -> HttpHeader {
        unsafe {
            let header_ptr = Cronet_UrlRequestParams_request_headers_at(self.ptr, index);
            HttpHeader { ptr: header_ptr }
        }
    }

    pub fn clear_headers(&self) {
        unsafe {
            Cronet_UrlRequestParams_request_headers_clear(self.ptr);
        }
    }

    /// Disables cache for the request. If context is not set up to use cache, this call has no effect.
    pub fn set_disable_cache(&self, disable: bool) {
        unsafe {
            Cronet_UrlRequestParams_disable_cache_set(self.ptr, disable);
        }
    }

    pub fn disable_cache(&self) -> bool {
        unsafe { Cronet_UrlRequestParams_disable_cache_get(self.ptr) }
    }

    /// Priority of the request.
    pub fn set_priority(&self, priority: RequestPriority) {
        unsafe {
            Cronet_UrlRequestParams_priority_set(
                self.ptr,
                priority as Cronet_UrlRequestParams_REQUEST_PRIORITY,
            );
        }
    }

    pub fn priority(&self) -> RequestPriority {
        unsafe {
            let priority = Cronet_UrlRequestParams_priority_get(self.ptr);
            RequestPriority::try_from(priority).unwrap()
        }
    }

    /// Upload data provider.
    /// Setting this value switches the method to `POST` if not explicitly set.
    /// Starting the request will fail if a `Content-Type` header is not set.
    pub fn set_upload_data_provider(&self, provider: UploadDataProvider) {
        unsafe {
            Cronet_UrlRequestParams_upload_data_provider_set(self.ptr, provider.ptr);
        }
    }

    pub fn upload_data_provider(&self) -> UploadDataProvider {
        unsafe {
            let ptr = Cronet_UrlRequestParams_upload_data_provider_get(self.ptr);
            UploadDataProvider { ptr }
        }
    }

    /// Upload data provider executor that will be used to invoke [UploadDataProvider].
    pub fn set_upload_data_executor(&self, executor: &Executor) {
        unsafe { Cronet_UrlRequestParams_upload_data_provider_executor_set(self.ptr, executor.ptr) }
    }

    pub fn upload_data_executor(&self) -> Executor {
        unsafe {
            let ptr = Cronet_UrlRequestParams_upload_data_provider_executor_get(self.ptr);
            Executor { ptr }
        }
    }

    /// Marks that the executors this request will use to notify callbacks (for
    /// [UploadDataProvider] and [crate::UrlRequestCallback] are intentionally performing
    /// inline execution without switching to another thread.
    ///
    /// **Warning:** This option makes it easy to accidentally block the network thread.
    /// It should not be used if your callbacks perform disk I/O, acquire locks, or call into
    /// other code you don't carefully control and audit.
    pub fn set_allow_direct_executor(&self, allow: bool) {
        unsafe {
            Cronet_UrlRequestParams_allow_direct_executor_set(self.ptr, allow);
        }
    }

    pub fn allow_direct_executor(&self) -> bool {
        unsafe { Cronet_UrlRequestParams_allow_direct_executor_get(self.ptr) }
    }

    /// Associates the annotation object with this request.
    /// May add more than one.
    ///
    /// Annotations are passed through to [crate::RequestFinishedInfoListener].
    pub fn add_annotation(&self, annotation: Annotation) {
        unsafe {
            Cronet_UrlRequestParams_annotations_add(self.ptr, annotation.ptr);
        }
    }

    pub fn annotations_size(&self) -> u32 {
        unsafe { Cronet_UrlRequestParams_annotations_size(self.ptr) }
    }

    pub fn annotation_at(&self, index: u32) -> Annotation {
        unsafe {
            let ptr = Cronet_UrlRequestParams_annotations_at(self.ptr, index);
            Annotation { ptr }
        }
    }

    pub fn clear_annotations(&self) {
        unsafe {
            Cronet_UrlRequestParams_annotations_clear(self.ptr);
        }
    }

    /// Sets a listener that gets invoked at the end of each request.
    ///
    /// The listener is invoked with the request finished info on `RequestFinishedExecutor`, which must be set.
    ///
    /// The listener is called before `UrlRequestCallbackHandler::on_canceled()`,
    /// `UrlRequestCallbackHandler::on_failed()`, or
    /// `UrlRequestCallbackHandler::on_succeeded()` is called.
    /// Note that if [crate::RequestFinishedInfoListener] runs the listener asynchronously, the actual
    /// call to the listener may happen after a [crate::UrlRequestCallbackHandler] method is called.
    ///
    /// Assuming the listener won't run again (there are no pending requests with the listener attached, either via Engine or `URLRequest`),
    /// the app may destroy it once its [crate::OnRequestFinishedFn] has started,
    /// even inside that method.
    pub fn set_request_finished_listener(
        &self,
        request_finished_listener: RequestFinishedInfoListener,
    ) {
        unsafe {
            Cronet_UrlRequestParams_request_finished_listener_set(
                self.ptr,
                request_finished_listener.ptr,
            );
        }
    }

    pub fn request_finished_listener(&self) -> RequestFinishedInfoListener {
        unsafe {
            let ptr = Cronet_UrlRequestParams_request_finished_listener_get(self.ptr);
            RequestFinishedInfoListener { ptr }
        }
    }

    /// Sets the [Executor] used to run the [RequestFinishedInfoListener].
    ///
    /// Similar to [RequestFinishedInfoListener], the app may destroy `RequestFinishedExecutor` in or after [crate::OnRequestFinishedFn].
    ///
    /// It's also okay to destroy `RequestFinishedExecutor` in or after one
    /// of `UrlRequestCallbackHandler::on_canceled()`, `UrlRequestCallbackHandler::on_failed()` or `UrlRequestCallbackHandler::on_succeeded()`.
    ///
    /// Of course, both of these are only true if `request_finished_executor` isn't being used for anything else that might start running in the future.
    pub fn set_request_finished_executor(&self, executor: Executor) {
        unsafe {
            Cronet_UrlRequestParams_request_finished_executor_set(self.ptr, executor.ptr);
        }
    }

    pub fn request_finished_executor(&self) -> Executor {
        unsafe {
            let ptr = Cronet_UrlRequestParams_request_finished_executor_get(self.ptr);
            Executor { ptr }
        }
    }

    /// Idempotency of the request, which determines if it is safe to enable `0-RTT` for the Cronet request.
    ///
    /// By default, `0-RTT` is only enabled for safe HTTP methods, i.e. `GET`, `HEAD`, `OPTIONS` and `TRACE`.
    /// For other methods, enabling `0-RTT` may cause security issues since a network observer can replay the request.
    ///
    /// If the request has any side effects, those effects can happen multiple times.
    /// It is only safe to enable the `0-RTT` if it is known that the request is idempotent.
    pub fn set_idempotency(&self, idempotency: Idempotency) {
        unsafe {
            Cronet_UrlRequestParams_idempotency_set(self.ptr, idempotency as u32);
        }
    }

    pub fn idempotency(&self) -> Idempotency {
        unsafe {
            let idempotency = Cronet_UrlRequestParams_idempotency_get(self.ptr);
            Idempotency::try_from(idempotency).unwrap()
        }
    }
}

impl Destroy for UrlRequestParams {
    fn destroy(&self) {
        unsafe {
            Cronet_UrlRequestParams_Destroy(self.ptr);
        }
    }
}

#[cfg(feature = "client")]
impl<T> From<Request<T>> for UrlRequestParams
where
    T: Into<Body>,
{
    fn from(request: Request<T>) -> Self {
        let (parts, body) = request.into_parts();
        let Parts {
            method, headers, ..
        } = parts;

        let request_parameters = UrlRequestParams::new();
        request_parameters.set_method(method.as_str());

        for (name, value) in &headers {
            let header = HttpHeader::new();
            header.set_name(name.as_str());
            header.set_value(value.to_str().unwrap());
            request_parameters.add_header(header);
        }

        let body: Body = body.into();
        if body.len().unwrap_or(0) > 0 {
            let body_handler = BodyUploadDataProvider::new(body, None); // TODO: support rewind
            let upload_data_provider = UploadDataProvider::new(body_handler);
            request_parameters.set_upload_data_provider(upload_data_provider);
        }

        request_parameters
    }
}

/// Enum representing the request priority for [crate::UrlRequest] parameters.
#[derive(Debug, PartialEq)]
pub enum RequestPriority {
    /// Lowest request priority.
    Idle = 0,

    /// Very low request priority.
    Lowest = 1,

    /// Low request priority.
    Low = 2,

    /// Medium request priority.
    /// This is the default priority given to the request.
    Medium = 3,

    /// Highest request priority.
    Highest = 4,
}

impl TryFrom<u32> for RequestPriority {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RequestPriority::Idle),
            1 => Ok(RequestPriority::Lowest),
            2 => Ok(RequestPriority::Low),
            3 => Ok(RequestPriority::Medium),
            4 => Ok(RequestPriority::Highest),
            _ => Err(()),
        }
    }
}

/// Enum representing idempotency options for URLRequest parameters.
#[derive(Debug, PartialEq)]
pub enum Idempotency {
    /// Default idempotency.
    DefaultIdempotency = 0,

    /// Idempotent request.
    Idempotent = 1,

    /// Non-idempotent request.
    NotIdempotent = 2,
}

impl TryFrom<u32> for Idempotency {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Idempotency::DefaultIdempotency),
            1 => Ok(Idempotency::Idempotent),
            2 => Ok(Idempotency::NotIdempotent),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        url_request_params::{Idempotency, RequestPriority},
        Annotation, Buffer, Destroy, Executor, HttpHeader, RequestFinishedInfoListener,
        UploadDataProvider, UploadDataProviderHandler, UploadDataSink, UrlRequestParams,
    };

    struct TestUploadDataProviderHandler;

    impl UploadDataProviderHandler for TestUploadDataProviderHandler {
        fn length(&self, _: UploadDataProvider) -> i64 {
            10
        }

        fn read(&self, _: UploadDataProvider, _: UploadDataSink, _: Buffer) {}

        fn rewind(&mut self, _: UploadDataProvider, sink: UploadDataSink) {
            sink.on_rewind_succeeded();
        }

        fn close(&self, upload_data_provider: UploadDataProvider) {
            upload_data_provider.destroy();
        }
    }

    #[test]
    fn test_url_request_params() {
        let url_request_params = UrlRequestParams::new();
        url_request_params.set_method("GET");
        assert_eq!(url_request_params.method(), "GET");
        let http_header = HttpHeader::new();
        http_header.set_name("User-Agent");
        http_header.set_value("Cronet");
        url_request_params.add_header(http_header);
        assert_eq!(url_request_params.header_size(), 1);
        let http_header = url_request_params.header_at(0);
        assert_eq!(http_header.name(), "User-Agent");
        assert_eq!(http_header.value(), "Cronet");
        url_request_params.clear_headers();
        assert_eq!(url_request_params.header_size(), 0);
        url_request_params.set_disable_cache(true);
        assert_eq!(url_request_params.disable_cache(), true);
        url_request_params.set_priority(RequestPriority::Lowest);
        assert_eq!(url_request_params.priority(), RequestPriority::Lowest);
        let upload_data_provider = UploadDataProvider::new(TestUploadDataProviderHandler);
        url_request_params.set_upload_data_provider(upload_data_provider);
        assert_eq!(url_request_params.upload_data_provider().length(), 10);
        url_request_params.set_upload_data_executor(&Executor::new(|_, _| {}));
        assert_eq!(
            url_request_params.upload_data_executor().ptr.is_null(),
            false
        );
        url_request_params.set_allow_direct_executor(true);
        assert_eq!(url_request_params.allow_direct_executor(), true);
        let annotation = Annotation::default();
        url_request_params.add_annotation(annotation);
        assert_eq!(url_request_params.annotations_size(), 1);
        url_request_params.clear_annotations();
        assert_eq!(url_request_params.annotations_size(), 0);
        url_request_params
            .set_request_finished_listener(RequestFinishedInfoListener::new(|_, _, _, _| {}));
        assert_eq!(
            url_request_params.request_finished_listener().ptr.is_null(),
            false
        );
        url_request_params.set_request_finished_executor(Executor::new(|_, _| {}));
        assert_eq!(
            url_request_params.request_finished_executor().ptr.is_null(),
            false
        );
        url_request_params.set_idempotency(Idempotency::Idempotent);
        assert_eq!(url_request_params.idempotency(), Idempotency::Idempotent);
        url_request_params.destroy();
    }
}
