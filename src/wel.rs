// SPDX-License-Identifier: Apache-2.0

// Copyright 2023 Joel E. Anderson
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use stumpless_sys::{
    stumpless_add_default_wel_event_source, stumpless_close_wel_target,
    stumpless_open_local_wel_target, stumpless_target,
};

use std::error::Error;
use std::ffi::CString;

use crate::error::last_error;
use crate::Target;

pub struct WelTarget {
    target: *mut stumpless_target,
}

impl WelTarget {
    pub fn new(log_name: &str) -> Result<Self, Box<dyn Error>> {
        let c_log_name = CString::new(log_name)?;
        let wel_target = unsafe { stumpless_open_local_wel_target(c_log_name.as_ptr()) };

        if wel_target.is_null() {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        } else {
            Ok(WelTarget { target: wel_target })
        }
    }
}

unsafe impl Sync for WelTarget {}

impl Target for WelTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

impl Drop for WelTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_wel_target(self.target);
        }
    }
}

pub fn add_default_wel_event_source() -> Result<u32, Box<dyn Error>> {
    let add_result = unsafe { stumpless_add_default_wel_event_source() };

    if add_result == 0 {
        Ok(add_result.try_into().unwrap())
    } else {
        match last_error() {
            Ok(_success) => panic!("inconsistent stumpless error state"),
            Err(err) => Err(Box::new(err)),
        }
    }
}
