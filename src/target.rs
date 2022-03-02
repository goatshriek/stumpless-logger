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

use crate::entry::Entry;
use crate::error::{last_error, StumplessError};
use std::error::Error;
use std::ffi::CString;
use stumpless_sys::{
    stumpless_add_entry, stumpless_add_message_str, stumpless_get_default_target,
    stumpless_open_target, stumpless_target,
};

pub trait Target: Sync {
    fn get_pointer(&self) -> *mut stumpless_target;

    fn add_entry(&self, entry: &Entry) -> Result<u32, StumplessError> {
        let add_result = unsafe { stumpless_add_entry(self.get_pointer(), entry.entry) };

        if add_result >= 0 {
            Ok(add_result.try_into().unwrap())
        } else {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(err),
            }
        }
    }

    fn add_message(&self, message: &str) -> Result<u32, Box<dyn Error>> {
        let c_message = CString::new(message)?;

        let add_result =
            unsafe { stumpless_add_message_str(self.get_pointer(), c_message.as_ptr()) };

        if add_result >= 0 {
            Ok(add_result.try_into().unwrap())
        } else {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        }
    }

    fn open(&self) -> Result<(), StumplessError> {
        let open_result = unsafe { stumpless_open_target(self.get_pointer()) };
        if open_result.is_null() {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(err),
            }
        } else {
            Ok(())
        }
    }
}

pub struct DefaultTarget {
    target: *mut stumpless_target,
}

impl DefaultTarget {
    pub fn get_default_target() -> Result<Self, Box<dyn Error>> {
        let default_target = unsafe { stumpless_get_default_target() };

        if default_target.is_null() {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        } else {
            Ok(DefaultTarget {
                target: default_target,
            })
        }
    }
}

unsafe impl Sync for DefaultTarget {}

impl Target for DefaultTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}
