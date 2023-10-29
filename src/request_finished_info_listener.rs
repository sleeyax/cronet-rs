use crate::{
    CronetError, Cronet_RequestFinishedInfoListenerPtr, RequestFinishedInfo, UrlResponseInfo,
};

pub struct RequestFinishedInfoListener {
    pub(crate) ptr: Cronet_RequestFinishedInfoListenerPtr,
}

pub type OnRequestFinishedFunc = fn(
    listener: RequestFinishedInfoListener,
    request_info: RequestFinishedInfo,
    response_info: UrlResponseInfo,
    error: CronetError,
);
