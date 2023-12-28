use std::{mem, sync::mpsc::Sender};

use bytes::BufMut;
use http::Response;

use crate::{
    Buffer, CronetError, Destroy, UrlRequest, UrlRequestCallback, UrlRequestCallbackHandler,
    UrlResponseInfo,
};

use super::Body;

pub type ShouldRedirectFn = fn(new_location_url: &str) -> bool;

#[derive(Debug)]
pub enum Status {
    Success(Response<Body>),
    Canceled,
    Error(CronetError),
}

pub struct ResponseHandler {
    should_redirect: ShouldRedirectFn,
    response: Response<Body>,
    tx: Sender<Status>,
    buffer: Option<Buffer>,
    buffer_size: u64,
}

impl ResponseHandler {
    pub fn new(should_redirect: ShouldRedirectFn, tx: Sender<Status>) -> Self {
        Self {
            should_redirect,
            response: Response::default(),
            tx,
            buffer: None,
            buffer_size: 512,
        }
    }

    /// Sets the buffer size for reading the response body.
    /// The default is `512` bytes.
    pub fn set_buffer_size(&mut self, buffer_size: u64) {
        self.buffer_size = buffer_size;
    }

    /// Continues reading the response body.
    fn read(&mut self, req: UrlRequest) {
        if let Some(old_buffer) = &self.buffer {
            old_buffer.destroy();
        }

        let buffer = Buffer::new_with_size(self.buffer_size);
        self.buffer = Some(Buffer { ptr: buffer.ptr });

        req.read(buffer);
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
        if (self.should_redirect)(new_location_url) {
            request.follow_redirect();
        } else {
            self.response = info.into();
        }
    }

    fn on_response_started(
        &mut self,
        _: UrlRequestCallback,
        req: UrlRequest,
        info: UrlResponseInfo,
    ) {
        self.response = info.into();
        self.read(req);
    }

    fn on_read_completed(
        &mut self,
        _: UrlRequestCallback,
        req: UrlRequest,
        _: UrlResponseInfo,
        buffer: Buffer,
        bytes_read: u64,
    ) {
        if bytes_read == 0 {
            return;
        }

        let data = buffer.data_slice::<u8>(bytes_read as usize);
        self.response.body_mut().as_bytes_mut().unwrap().put(data);

        self.read(req);
    }

    fn on_succeeded(&mut self, _: UrlRequestCallback, req: UrlRequest, _: UrlResponseInfo) {
        req.destroy();
        let response = mem::take(&mut self.response);
        self.tx.send(Status::Success(response)).unwrap();
    }

    fn on_failed(
        &mut self,
        _: UrlRequestCallback,
        req: UrlRequest,
        _: UrlResponseInfo,
        error: CronetError,
    ) {
        req.destroy();
        self.tx.send(Status::Error(error)).unwrap();
    }

    fn on_canceled(&mut self, _: UrlRequestCallback, req: UrlRequest, _: UrlResponseInfo) {
        req.destroy();
        self.tx.send(Status::Canceled).unwrap();
    }
}
