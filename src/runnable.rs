use once_cell::sync::Lazy;

use crate::{
    state::CronetCallbacks, Cronet_RunnablePtr, Cronet_Runnable_CreateWith,
    Cronet_Runnable_Destroy, Cronet_Runnable_Run, Destroy,
};

static mut RUNNABLE_CALLBACKS: Lazy<CronetCallbacks<Cronet_RunnablePtr, RunnableRunFn>> =
    Lazy::new(|| CronetCallbacks::new());

#[no_mangle]
unsafe extern "C" fn cronetRunnableOnRun(selfPtr: Cronet_RunnablePtr) {
    let lockedMap = RUNNABLE_CALLBACKS.map().lock().unwrap();
    if let Some(callback) = lockedMap.get(&selfPtr) {
        callback(Runnable { ptr: selfPtr });
    }
}

pub type RunnableRunFn = fn(runnable: Runnable);

/// An interface to run commands on the Executor.
///
/// Note: In general, creating Runnables should only be done by Cronet.
/// Runnables created by the app don't have the ability to perform operations when the [Runnable] is being destroyed (i.e. by `Cronet_Runnable_Destroy``)
/// so resource leaks are possible if the [Runnable] is posted to an [Executor] that is being shutdown with unexecuted Runnables.
/// In controlled testing environments deallocation of associated resources can be performed in `run()` if the runnable can be assumed to always be executed.
pub struct Runnable {
    pub(crate) ptr: Cronet_RunnablePtr,
}

impl Runnable {
    pub fn new(on_run: RunnableRunFn) -> Self {
        unsafe {
            let ptr: *mut crate::Cronet_Runnable =
                Cronet_Runnable_CreateWith(Some(cronetRunnableOnRun));
            RUNNABLE_CALLBACKS.map().lock().unwrap().insert(ptr, on_run);
            Self { ptr }
        }
    }

    pub fn run(&self) {
        unsafe {
            Cronet_Runnable_Run(self.ptr);
        }
    }
}

impl Destroy for Runnable {
    fn destroy(&self) {
        unsafe {
            let mut lockedMap = RUNNABLE_CALLBACKS.map().lock().unwrap();
            lockedMap.remove(&self.ptr);
            Cronet_Runnable_Destroy(self.ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Destroy;

    #[test]
    fn test_runnable() {
        let runnable = super::Runnable::new(|_| {});
        // TODO: test if the runnable actually executes the callback
        runnable.run();
        runnable.destroy();
    }
}
