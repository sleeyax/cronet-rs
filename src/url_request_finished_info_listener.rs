use crate::{
    CronetError, Cronet_RequestFinishedInfoListenerPtr, UrlRequestFinishedInfo, UrlResponseInfo,
};

pub struct UrlRequestFinishedInfoListener {
    pub(crate) ptr: Cronet_RequestFinishedInfoListenerPtr,
}

pub type UrlRequestFinishedInfoListenerOnRequestFinishedFunc = fn(
    listener: UrlRequestFinishedInfoListener,
    request_info: UrlRequestFinishedInfo,
    response_info: UrlResponseInfo,
    error: CronetError,
);
