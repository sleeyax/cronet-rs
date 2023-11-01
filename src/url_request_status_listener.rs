use once_cell::sync::Lazy;

use crate::{
    state::CronetCallbacks, Cronet_UrlRequestStatusListenerPtr,
    Cronet_UrlRequestStatusListener_CreateWith, Cronet_UrlRequestStatusListener_Destroy,
    Cronet_UrlRequestStatusListener_Status, Destroy,
};

static mut URL_REQUEST_STATUS_LISTENER_CALLBACKS: Lazy<
    CronetCallbacks<Cronet_UrlRequestStatusListenerPtr, UrlRequestStatusListenerOnStatusFn>,
> = Lazy::new(|| CronetCallbacks::new());

#[no_mangle]
unsafe extern "C" fn cronetUrlRequestStatusListenerOnStatus(
    self_ptr: Cronet_UrlRequestStatusListenerPtr,
    status_ptr: Cronet_UrlRequestStatusListener_Status,
) {
    let lockedMap = URL_REQUEST_STATUS_LISTENER_CALLBACKS.map().lock().unwrap();
    if let Some(callback) = lockedMap.get(&self_ptr) {
        callback(
            UrlRequestStatusListener { ptr: self_ptr },
            UrlRequestStatus::try_from(status_ptr).unwrap(),
        );
    }
}

pub type UrlRequestStatusListenerOnStatusFn =
    fn(url_request_status_listener: UrlRequestStatusListener, status: UrlRequestStatus);

pub struct UrlRequestStatusListener {
    pub(crate) ptr: Cronet_UrlRequestStatusListenerPtr,
}

impl UrlRequestStatusListener {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: Cronet_UrlRequestStatusListener_CreateWith(Some(
                    cronetUrlRequestStatusListenerOnStatus,
                )),
            }
        }
    }
}

impl Destroy for UrlRequestStatusListener {
    fn destroy(&self) {
        unsafe {
            let mut lockedMap = URL_REQUEST_STATUS_LISTENER_CALLBACKS.map().lock().unwrap();
            lockedMap.remove(&self.ptr);
            Cronet_UrlRequestStatusListener_Destroy(self.ptr);
        }
    }
}

/// Enum representing the status of a [crate::UrlRequest].
#[derive(Debug, PartialEq)]
pub enum UrlRequestStatus {
    /// This state indicates that the request is completed, canceled, or is not started.
    Invalid = -1,

    /// This state corresponds to a resource load that has either not yet begun
    /// or is idle waiting for the consumer to do something to move things along
    /// (e.g. when the consumer of a [crate::UrlRequest] has not called
    /// [crate::UrlRequest].read() yet).
    Idle = 0,

    /// When a socket pool group is below the maximum number of sockets allowed
    /// per group, but a new socket cannot be created due to the per-pool socket
    /// limit, this state is returned by all requests for the group waiting on an
    /// idle connection, except those that may be serviced by a pending new
    /// connection.
    WaitingForStalledSocketPool = 1,

    /// When a socket pool group has reached the maximum number of sockets
    /// allowed per group, this state is returned for all requests that don't
    /// have a socket, except those that correspond to a pending new connection.
    WaitingForAvailableSocket = 2,

    /// This state indicates that the [crate::UrlRequest] delegate has chosen to block
    /// this request before it was sent over the network.
    WaitingForDelegate = 3,

    /// This state corresponds to a resource load that is blocked waiting for
    /// access to a resource in the cache. If multiple requests are made for the
    /// same resource, the first request will be responsible for writing (or
    /// updating) the cache entry and the second request will be deferred until
    /// the first completes. This may be done to optimize for cache reuse.
    WaitingForCache = 4,

    /// This state corresponds to a resource being blocked waiting for the
    /// PAC script to be downloaded.
    DownloadingPacFile = 5,

    /// This state corresponds to a resource load that is blocked waiting for a
    /// proxy autoconfig script to return a proxy server to use.
    ResolvingProxyForUrl = 6,

    /// This state corresponds to a resource load that is blocked waiting for a
    /// proxy autoconfig script to return a proxy server to use, but that proxy
    /// script is busy resolving the IP address of a host.
    ResolvingHostInPacFile = 7,

    /// This state indicates that we're in the process of establishing a tunnel
    /// through the proxy server.
    EstablishingProxyTunnel = 8,

    /// This state corresponds to a resource load that is blocked waiting for a
    /// host name to be resolved. This could either indicate resolution of the
    /// origin server corresponding to the resource or to the host name of a
    /// proxy server used to fetch the resource.
    ResolvingHost = 9,

    /// This state corresponds to a resource load that is blocked waiting for a
    /// TCP connection (or other network connection) to be established. HTTP
    /// requests that reuse a keep-alive connection skip this state.
    Connecting = 10,

    /// This state corresponds to a resource load that is blocked waiting for the
    /// SSL handshake to complete.
    SslHandshake = 11,

    /// This state corresponds to a resource load that is blocked waiting to
    /// completely upload a request to a server. In the case of a HTTP POST
    /// request, this state includes the period of time during which the message
    /// body is being uploaded.
    SendingRequest = 12,

    /// This state corresponds to a resource load that is blocked waiting for the
    /// response to a network request. In the case of a HTTP transaction, this
    /// corresponds to the period after the request is sent and before all of the
    /// response headers have been received.
    WaitingForResponse = 13,

    /// This state corresponds to a resource load that is blocked waiting for a
    /// read to complete. In the case of a HTTP transaction, this corresponds to
    /// the period after the response headers have been received and before all
    /// of the response body has been downloaded. (NOTE: This state only applies
    /// for an [crate::UrlRequest] while there is an outstanding
    /// [crate::UrlRequest] `read()` operation.)
    ReadingResponse = 14,
}

impl TryFrom<i32> for UrlRequestStatus {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::Invalid),
            0 => Ok(Self::Idle),
            1 => Ok(Self::WaitingForStalledSocketPool),
            2 => Ok(Self::WaitingForAvailableSocket),
            3 => Ok(Self::WaitingForDelegate),
            4 => Ok(Self::WaitingForCache),
            5 => Ok(Self::DownloadingPacFile),
            6 => Ok(Self::ResolvingProxyForUrl),
            7 => Ok(Self::ResolvingHostInPacFile),
            8 => Ok(Self::EstablishingProxyTunnel),
            9 => Ok(Self::ResolvingHost),
            10 => Ok(Self::Connecting),
            11 => Ok(Self::SslHandshake),
            12 => Ok(Self::SendingRequest),
            13 => Ok(Self::WaitingForResponse),
            14 => Ok(Self::ReadingResponse),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Destroy, UrlRequestStatusListener};

    #[test]
    fn test_url_request_status_listener() {
        let listener = UrlRequestStatusListener::new();
        assert!(listener.ptr != std::ptr::null_mut());
        listener.destroy();
    }
}
