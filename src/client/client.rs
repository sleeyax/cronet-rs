use std::thread;

use crate::{Destroy, Engine, EngineParams, Executor, UrlRequestParams};

use super::Body;

type ShouldRedirectFn = fn(new_location_url: &str) -> bool;

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

    pub fn send<T>(&self, request: http::Request<Body>) -> http::Result<http::Response<Body>> {
        let request_parameters = UrlRequestParams::from(request);
        request_parameters.set_upload_data_executor(&self.executor);

        // TODO: implement response handler

        http::Result::Ok(http::Response::new(Body::from("")))
    }
}
