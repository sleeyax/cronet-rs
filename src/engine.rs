use std::ffi::{CStr, CString};

use crate::{
    Cronet_EnginePtr, Cronet_Engine_Create, Cronet_Engine_Destroy,
    Cronet_Engine_GetDefaultUserAgent, Cronet_Engine_GetVersionString, Cronet_Engine_Shutdown,
    Cronet_Engine_StartNetLogToFile, Cronet_Engine_StartWithParams, Cronet_Engine_StopNetLog,
    Destroy, EngineParams, EngineResult,
};

pub struct Engine {
    pub(crate) ptr: Cronet_EnginePtr,
}

impl Engine {
    pub fn new() -> Self {
        unsafe {
            Engine {
                ptr: Cronet_Engine_Create(),
            }
        }
    }

    /// Start the Engine with the given [EngineParams].
    /// The engine must be started once and only once before other methods can be used.
    ///
    /// Please note that the [EngineParams] will be destroyed after the engine is started.
    pub fn start(&self, params: EngineParams) -> EngineResult {
        unsafe {
            let result = Cronet_Engine_StartWithParams(self.ptr, params.ptr);
            params.destroy();
            EngineResult::try_from(result).unwrap()
        }
    }

    /// Start NetLog logging to a file.
    /// The NetLog will contain events emitted by all live Engines.
    /// The NetLog is useful for debugging.
    /// The file can be viewed using a Chrome browser navigated to `chrome://net-internals/#import`.
    ///
    /// Arguments:
    ///
    /// * `file_name`: The complete file path.
    ///                It must not be empty and if the file exists, it is truncated before starting.
    ///                If actively logging, this method is ignored.
    /// * `log_all`:   Include basic events, user cookies, credentials and all transferred bytes in the log.
    ///                This option presents a privacy risk, since it exposes the user's credentials, and should only be used with the user's consent and in situations where the log won't be public.
    ///                Set to `false` to only include basic events.
    ///
    /// Returns `true` if netlog has started successfully, `false` otherwise.
    pub fn start_net_log(&self, file_name: &str, log_all: bool) {
        unsafe {
            let file_name = CString::new(file_name).unwrap();
            Cronet_Engine_StartNetLogToFile(self.ptr, file_name.as_ptr(), log_all);
        }
    }

    /// Stop NetLog logging and flushes file to disk.
    /// If a logging session is not in progress, this call is ignored.
    /// This method blocks until the log is closed to ensure that log file is complete and available.
    pub fn stop_net_log(&self) {
        unsafe {
            Cronet_Engine_StopNetLog(self.ptr);
        }
    }

    /// Shut down the Engine if there are no active requests, otherwise returns a failure Result.
    ///
    /// Note that this method cannot be called on a network thread - the thread Cronet calls into [Executor] (which is different from the thread the Executor invokes callbacks on).
    /// This method blocks until all the Engine's resources have been cleaned up.
    pub fn shutdown(&self) -> EngineResult {
        unsafe {
            let result = Cronet_Engine_Shutdown(self.ptr);
            EngineResult::try_from(result).unwrap()
        }
    }

    /// A human-readable version string of the engine.
    pub fn version(&self) -> &str {
        unsafe {
            let version = Cronet_Engine_GetVersionString(self.ptr);
            let version = CStr::from_ptr(version);
            version.to_str().unwrap()
        }
    }

    /// Returns the default value of the `User-Agent` header.
    /// Can be accessed before `StartWithParams()` is called.
    pub fn default_user_agent(&self) -> &str {
        unsafe {
            let version = Cronet_Engine_GetDefaultUserAgent(self.ptr);
            let version = CStr::from_ptr(version);
            version.to_str().unwrap()
        }
    }

    pub fn add_request_finished_listener(&self) {
        unimplemented!() // TODO: implement me
    }

    pub fn remove_request_finished_listener(&self) {
        unimplemented!() // TODO: implement me
    }
}

impl Destroy for Engine {
    fn destroy(&self) {
        unsafe {
            Cronet_Engine_Destroy(self.ptr);
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Destroy;

    #[test]
    fn it_gets_version() {
        let engine = super::Engine::new();
        let version = engine.version();
        assert!(!version.is_empty());
        engine.destroy();
    }

    #[test]
    fn it_gets_default_user_agent() {
        let engine = super::Engine::new();
        let default_user_agent = engine.default_user_agent();
        assert!(!default_user_agent.is_empty());
        engine.destroy();
    }

    #[test]
    fn it_starts_and_stops_engine() {
        let engine = super::Engine::new();
        let params = crate::EngineParams::new();
        let result = engine.start(params);
        assert_eq!(result, crate::EngineResult::Success);
        let result = engine.shutdown();
        assert_eq!(result, crate::EngineResult::Success);
        engine.destroy();
    }

    #[test]
    fn test_engine() {
        let engine = super::Engine::new();
        // TODO: add full E2E engine test
        engine.destroy();
    }
}
