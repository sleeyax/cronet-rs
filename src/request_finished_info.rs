use crate::{
    Annotation, Cronet_RequestFinishedInfoPtr, Cronet_RequestFinishedInfo_Create,
    Cronet_RequestFinishedInfo_Destroy, Cronet_RequestFinishedInfo_annotations_add,
    Cronet_RequestFinishedInfo_annotations_at, Cronet_RequestFinishedInfo_annotations_clear,
    Cronet_RequestFinishedInfo_annotations_size, Cronet_RequestFinishedInfo_finished_reason_get,
    Cronet_RequestFinishedInfo_finished_reason_set, Cronet_RequestFinishedInfo_metrics_get,
    Cronet_RequestFinishedInfo_metrics_move, Cronet_RequestFinishedInfo_metrics_set, Destroy,
    Metrics,
};

pub struct RequestFinishedInfo {
    pub(crate) ptr: Cronet_RequestFinishedInfoPtr,
}

impl RequestFinishedInfo {
    pub fn new() -> Self {
        unsafe {
            RequestFinishedInfo {
                ptr: Cronet_RequestFinishedInfo_Create(),
            }
        }
    }

    /// Set the [Metrics] collected for this request.
    pub fn set_metrics(&self, metrics: Metrics) {
        unsafe {
            Cronet_RequestFinishedInfo_metrics_set(self.ptr, metrics.ptr);
        }
    }

    pub fn move_metrics(&self, metrics: Metrics) {
        unsafe { Cronet_RequestFinishedInfo_metrics_move(self.ptr, metrics.ptr) }
    }

    /// Get the [Metrics] collected for this request.
    pub fn metrics(&self) -> Metrics {
        unsafe {
            let ptr = Cronet_RequestFinishedInfo_metrics_get(self.ptr);
            Metrics { ptr }
        }
    }

    pub fn add_annotation(&self, annotation: Annotation) {
        unsafe {
            Cronet_RequestFinishedInfo_annotations_add(self.ptr, annotation.ptr);
        }
    }

    pub fn clear_annotations(&self) {
        unsafe {
            Cronet_RequestFinishedInfo_annotations_clear(self.ptr);
        }
    }

    /// The objects that the caller has supplied when initiating the request, using [crate::UrlRequestParams::add_annotation].
    ///
    /// Annotations can be used to associate a [RequestFinishedInfo] with the original request or type of request.
    pub fn annotations_size(&self) -> u32 {
        unsafe { Cronet_RequestFinishedInfo_annotations_size(self.ptr) }
    }

    pub fn annotation_at(&self, index: u32) -> Annotation {
        unsafe {
            let ptr = Cronet_RequestFinishedInfo_annotations_at(self.ptr, index);
            Annotation { ptr }
        }
    }

    /// Returns the reason why the request finished.
    pub fn finished_reason(&self) -> RequestFinishedInfoReason {
        unsafe {
            let reason = Cronet_RequestFinishedInfo_finished_reason_get(self.ptr);
            reason.try_into().unwrap()
        }
    }

    pub fn set_finished_reason(&self, reason: RequestFinishedInfoReason) {
        unsafe {
            Cronet_RequestFinishedInfo_finished_reason_set(self.ptr, reason as u32);
        }
    }
}

impl Destroy for RequestFinishedInfo {
    fn destroy(&self) {
        unsafe { Cronet_RequestFinishedInfo_Destroy(self.ptr) }
    }
}

/// Enum representing the reason why the request finished.
#[derive(Debug, PartialEq)]
pub enum RequestFinishedInfoReason {
    /// The request succeeded.
    Succeeded = 0,

    /// The request failed or returned an error.
    Failed = 1,

    /// The request was canceled.
    Canceled = 2,
}

impl TryFrom<u32> for RequestFinishedInfoReason {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RequestFinishedInfoReason::Succeeded),
            1 => Ok(RequestFinishedInfoReason::Failed),
            2 => Ok(RequestFinishedInfoReason::Canceled),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Destroy, RequestFinishedInfoReason};

    #[test]
    fn it_gets_metrics() {
        let url_request_finished_info = super::RequestFinishedInfo::new();
        let metrics = super::Metrics::new();
        metrics.set_received_byte_count(100);
        url_request_finished_info.set_metrics(metrics);
        let metrics2 = url_request_finished_info.metrics();
        assert_eq!(metrics2.received_byte_count(), 100);
        url_request_finished_info.destroy();
    }

    #[test]
    fn it_gets_annotations() {
        let url_request_finished_info = super::RequestFinishedInfo::new();
        let annotation = super::Annotation::default();
        let annotation_ptr = annotation.ptr.clone();
        url_request_finished_info.add_annotation(annotation);
        let annotations_size = url_request_finished_info.annotations_size();
        assert_eq!(annotations_size, 1);
        let annotation2 = url_request_finished_info.annotation_at(0);
        assert_eq!(annotation_ptr, annotation2.ptr);
        url_request_finished_info.clear_annotations();
        let annotations_size = url_request_finished_info.annotations_size();
        assert_eq!(annotations_size, 0);
        url_request_finished_info.destroy();
    }

    #[test]
    fn it_gets_finished_reason() {
        let url_request_finished_info = super::RequestFinishedInfo::new();
        url_request_finished_info.set_finished_reason(super::RequestFinishedInfoReason::Canceled);
        let finished_reason = url_request_finished_info.finished_reason();
        assert_eq!(finished_reason, RequestFinishedInfoReason::Canceled);
        url_request_finished_info.destroy();
    }
}
