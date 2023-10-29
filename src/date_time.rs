use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    Cronet_DateTimePtr, Cronet_DateTime_Create, Cronet_DateTime_Destroy, Cronet_DateTime_value_get,
    Cronet_DateTime_value_set, Destroy,
};

pub struct DateTime {
    pub(crate) ptr: Cronet_DateTimePtr,
}

impl DateTime {
    pub fn new() -> Self {
        unsafe {
            DateTime {
                ptr: Cronet_DateTime_Create(),
            }
        }
    }

    pub fn set_millis(&self, milliseconds: i64) {
        unsafe {
            Cronet_DateTime_value_set(self.ptr, milliseconds);
        }
    }

    pub fn get_millis(&self) -> i64 {
        unsafe { Cronet_DateTime_value_get(self.ptr) }
    }

    pub fn set(&self, time: SystemTime) {
        let duration = time.duration_since(UNIX_EPOCH).unwrap();
        let milliseconds = duration.as_millis();
        self.set_millis(milliseconds as i64);
    }

    pub fn get(&self) -> SystemTime {
        let milliseconds = self.get_millis();
        let duration = Duration::from_millis(milliseconds as u64);
        UNIX_EPOCH + duration
    }
}

impl Destroy for DateTime {
    fn destroy(&self) {
        unsafe { Cronet_DateTime_Destroy(self.ptr) }
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use crate::Destroy;

    #[test]
    fn test_date_time() {
        let date_time = super::DateTime::new();
        let now = std::time::SystemTime::now();
        let now_millis = now.duration_since(UNIX_EPOCH).unwrap().as_millis();
        date_time.set(now);
        let now2 = date_time.get();
        let now2_millis = now2.duration_since(UNIX_EPOCH).unwrap().as_millis();
        assert_eq!(now_millis, now2_millis);
        date_time.destroy();
    }

    #[test]
    fn test_date_time_millis() {
        let millis: i64 = 1698591535578;
        let date_time = super::DateTime::new();
        date_time.set_millis(millis);
        let millis2 = date_time.get_millis();
        assert_eq!(millis, millis2);
        date_time.destroy();
    }
}
