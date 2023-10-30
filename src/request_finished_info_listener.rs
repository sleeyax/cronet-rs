use once_cell::sync::Lazy;

use crate::{
    state::CronetCallbacks, CronetError, Cronet_ErrorPtr, Cronet_RawDataPtr,
    Cronet_RequestFinishedInfoListenerPtr, Cronet_RequestFinishedInfoListener_CreateWith,
    Cronet_RequestFinishedInfoListener_Destroy,
    Cronet_RequestFinishedInfoListener_SetClientContext, Cronet_RequestFinishedInfoPtr,
    Cronet_UrlResponseInfoPtr, Destroy, RequestFinishedInfo, UrlResponseInfo,
};

static mut REQUEST_FINISHED_INFO_LISTENER_CALLBACKS: Lazy<
    CronetCallbacks<Cronet_RequestFinishedInfoListenerPtr, OnRequestFinishedFunc>,
> = Lazy::new(|| CronetCallbacks::new());

#[no_mangle]
unsafe extern "C" fn cronetOnRequestFinished(
    selfPtr: Cronet_RequestFinishedInfoListenerPtr,
    request_info: Cronet_RequestFinishedInfoPtr,
    response_info: Cronet_UrlResponseInfoPtr,
    error: Cronet_ErrorPtr,
) {
    let mut lockedMap = REQUEST_FINISHED_INFO_LISTENER_CALLBACKS
        .map()
        .lock()
        .unwrap();
    lockedMap.remove(&selfPtr);
    if let Some(callback) = lockedMap.get(&selfPtr) {
        callback(
            RequestFinishedInfoListener { ptr: selfPtr },
            RequestFinishedInfo { ptr: request_info },
            UrlResponseInfo { ptr: response_info },
            CronetError { ptr: error },
        );
    }
}

pub struct RequestFinishedInfoListener {
    pub(crate) ptr: Cronet_RequestFinishedInfoListenerPtr,
}

impl RequestFinishedInfoListener {
    pub fn new(on_request_finished: OnRequestFinishedFunc) -> Self {
        unsafe {
            let ptr = Cronet_RequestFinishedInfoListener_CreateWith(Some(cronetOnRequestFinished));
            REQUEST_FINISHED_INFO_LISTENER_CALLBACKS
                .map()
                .lock()
                .unwrap()
                .insert(ptr, on_request_finished);
            Self { ptr }
        }
    }

    pub fn set_client_context(&self, raw_data: Cronet_RawDataPtr) {
        unsafe {
            Cronet_RequestFinishedInfoListener_SetClientContext(self.ptr, raw_data);
        }
    }
}

impl Destroy for RequestFinishedInfoListener {
    fn destroy(&self) {
        unsafe {
            Cronet_RequestFinishedInfoListener_Destroy(self.ptr);
        }
    }
}

pub type OnRequestFinishedFunc = fn(
    listener: RequestFinishedInfoListener,
    request_info: RequestFinishedInfo,
    response_info: UrlResponseInfo,
    error: CronetError,
);

#[cfg(test)]
mod tests {
    use crate::Destroy;

    #[test]
    fn it_sets_client_context() {
        let listener = super::RequestFinishedInfoListener::new(|_, _, _, _| {});
        listener.set_client_context(std::ptr::null_mut());
        listener.destroy();
    }
}
