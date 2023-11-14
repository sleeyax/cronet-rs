use std::thread;

use crate::{Destroy, Engine, EngineParams, Executor};

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

impl Client {
    pub fn new() -> Self {
        Self::new_with_redirect(|_| true)
    }

    pub fn new_with_redirect(should_redirect: ShouldRedirectFn) -> Self {
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
            should_redirect,
        }
    }

    pub fn send(&self, request: http::Request<()>) -> http::Result<http::Response<()>> {
        // TODO: continue implementation
        http::Result::Ok(http::Response::new(()))
    }
}
