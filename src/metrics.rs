use crate::{
    Cronet_MetricsPtr, Cronet_Metrics_Create, Cronet_Metrics_Destroy,
    Cronet_Metrics_connect_end_get, Cronet_Metrics_connect_end_set,
    Cronet_Metrics_connect_start_get, Cronet_Metrics_connect_start_set, Cronet_Metrics_dns_end_get,
    Cronet_Metrics_dns_end_set, Cronet_Metrics_dns_start_get, Cronet_Metrics_dns_start_set,
    Cronet_Metrics_push_end_get, Cronet_Metrics_push_end_set, Cronet_Metrics_push_start_get,
    Cronet_Metrics_push_start_set, Cronet_Metrics_received_byte_count_get,
    Cronet_Metrics_received_byte_count_set, Cronet_Metrics_request_end_get,
    Cronet_Metrics_request_end_set, Cronet_Metrics_request_start_get,
    Cronet_Metrics_request_start_set, Cronet_Metrics_response_start_get,
    Cronet_Metrics_response_start_set, Cronet_Metrics_sending_end_get,
    Cronet_Metrics_sending_end_set, Cronet_Metrics_sending_start_get,
    Cronet_Metrics_sending_start_set, Cronet_Metrics_sent_byte_count_get,
    Cronet_Metrics_sent_byte_count_set, Cronet_Metrics_socket_reused_get,
    Cronet_Metrics_socket_reused_set, Cronet_Metrics_ssl_end_get, Cronet_Metrics_ssl_end_set,
    Cronet_Metrics_ssl_start_get, Cronet_Metrics_ssl_start_set, DateTime, Destroy,
};

/// Metrics
/// Represents metrics collected for a single request. Most of these metrics are
/// timestamps for events during the lifetime of the request, which can be used
/// to build a detailed timeline for investigating performance.
///
/// Represents metrics collected for a single request. Most of these metrics are
/// timestamps for events during the lifetime of the request, which can be used
/// to build a detailed timeline for investigating performance.
///
/// Events happen in this order:
/// 1. request_start - request start
/// 2. dns_start - DNS start
/// 3. dns_end - DNS end
/// 4. connect_start - connect start
/// 5. ssl_start - SSL start
/// 6. ssl_end - SSL end
/// 7. connect_end - connect end
/// 8. sending_start - sending start
/// 9. sending_end - sending end
/// 10. response_start - response start
/// 11. request_end - request end
///
/// Start times are reported as the time when a request started blocking on the
/// event, not when the event actually occurred, with the exception of push
/// start and end. If a metric is not meaningful or not available, including
/// cases when a request finished before reaching that stage, start and end
/// times will be null. If no time was spent blocking on an event, start and end
/// will be the same time.
///
/// Timestamps are recorded using a clock that is guaranteed not to run
/// backwards. All timestamps are correct relative to the system clock at the
/// time of request start, and taking the difference between two timestamps will
/// give the correct difference between the events. In order to preserve this
/// property, timestamps for events other than request start are not guaranteed
/// to match the system clock at the times they represent.
///
/// Most timing metrics are taken from
/// [LoadTimingInfo](https://cs.chromium.org/chromium/src/net/base/load_timing_info.h),
/// which holds the information for [Navigation Timing](http://w3c.github.io/navigation-timing/)
/// and [Resource Timing](https://www.w3.org/TR/resource-timing/).
pub struct Metrics {
    pub(crate) ptr: Cronet_MetricsPtr,
}

impl Metrics {
    pub fn new() -> Self {
        unsafe {
            Metrics {
                ptr: Cronet_Metrics_Create(),
            }
        }
    }

    pub fn request_start(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_request_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn dns_start(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_dns_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn dns_end(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_dns_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn connect_start(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_connect_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn connect_end(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_connect_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn ssl_start(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_ssl_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn ssl_end(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_ssl_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn sending_start(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_sending_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn sending_end(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_sending_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn push_start(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_push_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn push_end(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_push_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn response_start(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_response_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn response_end(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_request_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn socket_reused(&self) -> bool {
        unsafe { Cronet_Metrics_socket_reused_get(self.ptr) }
    }

    pub fn sent_byte_count(&self) -> i64 {
        unsafe { Cronet_Metrics_sent_byte_count_get(self.ptr) }
    }

    pub fn received_byte_count(&self) -> i64 {
        unsafe { Cronet_Metrics_received_byte_count_get(self.ptr) }
    }

    pub fn set_request_start(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_request_start_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_dns_start(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_dns_start_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_dns_end(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_dns_end_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_connect_start(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_connect_start_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_connect_end(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_connect_end_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_ssl_start(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_ssl_start_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_ssl_end(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_ssl_end_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_sending_start(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_sending_start_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_sending_end(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_sending_end_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_push_start(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_push_start_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_push_end(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_push_end_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_response_start(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_response_start_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_response_end(&self, datetime: DateTime) {
        unsafe {
            Cronet_Metrics_request_end_set(self.ptr, datetime.ptr);
        }
    }

    pub fn set_socket_reused(&self, reused: bool) {
        unsafe {
            Cronet_Metrics_socket_reused_set(self.ptr, reused);
        }
    }

    pub fn set_sent_byte_count(&self, count: i64) {
        unsafe {
            Cronet_Metrics_sent_byte_count_set(self.ptr, count);
        }
    }

    pub fn set_received_byte_count(&self, count: i64) {
        unsafe {
            Cronet_Metrics_received_byte_count_set(self.ptr, count);
        }
    }
}

impl Destroy for Metrics {
    fn destroy(&self) {
        unsafe { Cronet_Metrics_Destroy(self.ptr) }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use crate::{DateTime, Destroy};

    #[test]
    fn test_metrics() {
        let metrics = super::Metrics::new();
        let now = DateTime::new();
        now.set(SystemTime::now());

        metrics.set_request_start(now);
        metrics.set_dns_start(now);
        metrics.set_dns_end(now);
        metrics.set_connect_start(now);
        metrics.set_connect_end(now);
        metrics.set_ssl_start(now);
        metrics.set_ssl_end(now);
        metrics.set_sending_start(now);
        metrics.set_sending_end(now);
        metrics.set_push_start(now);
        metrics.set_push_end(now);
        metrics.set_response_start(now);
        metrics.set_response_end(now);
        metrics.set_socket_reused(true);
        metrics.set_sent_byte_count(100);
        metrics.set_received_byte_count(100);

        assert_eq!(metrics.connect_start().millis(), now.millis());
        assert_eq!(metrics.connect_end().millis(), now.millis());
        assert_eq!(metrics.ssl_start().millis(), now.millis());
        assert_eq!(metrics.ssl_end().millis(), now.millis());
        assert_eq!(metrics.sending_start().millis(), now.millis());
        assert_eq!(metrics.sending_end().millis(), now.millis());
        assert_eq!(metrics.push_start().millis(), now.millis());
        assert_eq!(metrics.push_end().millis(), now.millis());
        assert_eq!(metrics.response_start().millis(), now.millis());
        assert_eq!(metrics.response_end().millis(), now.millis());
        assert_eq!(metrics.socket_reused(), true);
        assert_eq!(metrics.sent_byte_count(), 100);
        assert_eq!(metrics.received_byte_count(), 100);
        assert_eq!(metrics.request_start().millis(), now.millis());
        assert_eq!(metrics.dns_start().millis(), now.millis());
        assert_eq!(metrics.dns_end().millis(), now.millis());

        metrics.destroy();
    }
}
