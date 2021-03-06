use stumpless_sys::*;

use std::error::Error;
use std::ffi::CString;

use crate::StumplessError;
use crate::Target;

pub struct JournaldTarget {
    target: *mut stumpless_target,
}

impl JournaldTarget {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let target_name = CString::new("stumpless-cli")?;
        let journald_target = unsafe { stumpless_open_journald_target(target_name.as_ptr()) };

        if journald_target.is_null() {
            Err(Box::new(StumplessError))
        } else {
            Ok(JournaldTarget {
                target: journald_target,
            })
        }
    }
}

impl Target for JournaldTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

impl Drop for JournaldTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_journald_target(self.target);
        }
    }
}
