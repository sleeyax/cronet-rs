use once_cell::sync::Lazy;

use crate::{
    state::CronetCallbacks, Cronet_ExecutorPtr, Cronet_Executor_CreateWith,
    Cronet_Executor_Destroy, Cronet_Executor_Execute, Cronet_RunnablePtr, Destroy, Runnable,
};

static mut EXECUTOR_CALLBACKS: Lazy<CronetCallbacks<Cronet_ExecutorPtr, ExecutorExecuteFn>> =
    Lazy::new(CronetCallbacks::new);

#[no_mangle]
unsafe extern "C" fn cronetExecutorOnExecute(
    selfPtr: Cronet_ExecutorPtr,
    runnablePtr: Cronet_RunnablePtr,
) {
    let lockedMap = EXECUTOR_CALLBACKS.map().lock().unwrap();
    if let Some(callback) = lockedMap.get(&selfPtr) {
        callback(Executor { ptr: selfPtr }, Runnable { ptr: runnablePtr });
    }
}

pub type ExecutorExecuteFn = fn(executor: Executor, runnable: Runnable);

/// An interface provided by the app to run a [Runnable] asynchronously.
pub struct Executor {
    pub(crate) ptr: Cronet_ExecutorPtr,
}

impl Executor {
    pub fn new(on_execute: ExecutorExecuteFn) -> Self {
        unsafe {
            let ptr = Cronet_Executor_CreateWith(Some(cronetExecutorOnExecute));
            EXECUTOR_CALLBACKS
                .map()
                .lock()
                .unwrap()
                .insert(ptr, on_execute);
            Self { ptr }
        }
    }

    pub fn execute(&self, runnable: Runnable) {
        unsafe {
            Cronet_Executor_Execute(self.ptr, runnable.ptr);
        }
    }
}

impl Destroy for Executor {
    fn destroy(&self) {
        unsafe {
            let mut lockedMap = EXECUTOR_CALLBACKS.map().lock().unwrap();
            lockedMap.remove(&self.ptr);
            Cronet_Executor_Destroy(self.ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Destroy;

    #[test]
    fn test_executor() {
        // TODO: test if the executor actually runs the runnable
        let executor = super::Executor::new(|_, _| {});
        let runnable = super::Runnable::new(|_| {});
        executor.execute(runnable);
        executor.destroy();
    }
}
