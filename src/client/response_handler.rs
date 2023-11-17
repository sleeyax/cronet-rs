use std::sync::mpsc::Sender;

use bytes::BufMut;
use http::Response;

use crate::{
    Buffer, CronetError, UrlRequest, UrlRequestCallback, UrlRequestCallbackHandler, UrlResponseInfo,
};

use super::Body;

pub type ShouldRedirectFn = fn(new_location_url: &str) -> bool;

#[derive(Debug)]
pub enum Status {
    Success,
    Canceled,
    Error(CronetError),
}

pub struct ResponseHandler {
    pub should_redirect: ShouldRedirectFn,
    response: Response<Body>,
    tx: Sender<(Response<Body>, Status)>,
}

impl ResponseHandler {
    pub fn new(should_redirect: ShouldRedirectFn, tx: Sender<(Response<Body>, Status)>) -> Self {
        Self {
            should_redirect,
            response: Response::default(),
            tx,
        }
    }
}

impl UrlRequestCallbackHandler for ResponseHandler {
    fn on_redirect_received(
        &mut self,
        _: UrlRequestCallback,
        request: UrlRequest,
        info: UrlResponseInfo,
        new_location_url: &str,
    ) {
        let redirect = (self.should_redirect)(new_location_url);

        if !redirect {
            self.response = info.into();
            return;
        }

        request.follow_redirect();
    }

    fn on_response_started(&mut self, _: UrlRequestCallback, _: UrlRequest, info: UrlResponseInfo) {
        self.response = info.into();
    }

    fn on_read_completed(
        &mut self,
        _: UrlRequestCallback,
        _: UrlRequest,
        _: UrlResponseInfo,
        buffer: Buffer,
        bytes_read: u64,
    ) {
        if bytes_read == 0 {
            return;
        }

        let data = buffer.data::<&[u8]>();
        self.response.body_mut().as_bytes_mut().unwrap().put(data);
    }

    fn on_succeeded(&mut self, _: UrlRequestCallback, _: UrlRequest, _: UrlResponseInfo) {
        // TODO: fix lifetime of response to sender
        self.tx
            .send((Response::default(), Status::Success))
            .unwrap();
    }

    fn on_failed(
        &mut self,
        _: UrlRequestCallback,
        _: UrlRequest,
        _: UrlResponseInfo,
        error: CronetError,
    ) {
        // TODO: fix lifetime of response to sender
        self.tx
            .send((Response::default(), Status::Error(error)))
            .unwrap();
    }

    fn on_canceled(&mut self, _: UrlRequestCallback, _: UrlRequest, _: UrlResponseInfo) {
        // TODO: fix lifetime of response to sender
        self.tx
            .send((Response::default(), Status::Canceled))
            .unwrap();
    }
}
