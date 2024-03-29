use std::{sync::mpsc, thread};

use crate::{
    client::ClientError, Destroy, Engine, EngineParams, EngineResult, Executor, UrlRequest,
    UrlRequestCallback, UrlRequestParams,
};

use super::{Body, ResponseHandler, ShouldRedirectFn, Status};

pub struct Client {
    pub should_redirect: ShouldRedirectFn,
    pub engine: Engine,
    pub executor: Executor,
}

impl Destroy for Client {
    fn destroy(&self) {
        self.engine.shutdown();
        self.engine.destroy();
        self.executor.destroy();
    }
}

#[allow(dead_code)]
impl Client {
    pub fn new() -> Self {
        let engine_params = EngineParams::new();
        engine_params.set_enable_http_2(true);
        engine_params.set_enable_quic(true);
        engine_params.set_enable_brotli(true);
        engine_params.set_user_agent("cronet");

        let engine = Engine::new();
        engine.start(engine_params);

        let executor = Executor::new(|_, runnable| {
            thread::spawn(move || {
                runnable.run();
                runnable.destroy();
            });
        });

        Self {
            engine,
            executor,
            should_redirect: |_| true,
        }
    }

    /// Sets the function that determines whether a redirect should be followed.
    pub fn set_should_redirect(&mut self, should_redirect: ShouldRedirectFn) {
        self.should_redirect = should_redirect;
    }

    pub fn send(&self, request: http::Request<Body>) -> Result<http::Response<Body>, ClientError> {
        let uri = request.uri().to_string();

        let request_parameters = UrlRequestParams::from(request);
        request_parameters.set_upload_data_executor(&self.executor);

        let (tx, rx) = mpsc::channel::<Status>();
        let response_handler = ResponseHandler::new(self.should_redirect, tx);
        let callback = UrlRequestCallback::new(response_handler);
        let url_request = UrlRequest::new();
        url_request.init_with_params(
            &self.engine,
            uri.as_str(),
            &request_parameters,
            &callback,
            &self.executor,
        );
        // request_parameters.destroy();
        let result = url_request.start();
        if result != EngineResult::Success {
            return Result::Err(ClientError::EngineError(result));
        }

        let status = rx.recv().unwrap();

        match status {
            Status::Success(res) => Result::Ok(res),
            Status::Canceled => Result::Err(ClientError::CancellationError),
            Status::Error(e) => Result::Err(ClientError::CronetError(e)),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
