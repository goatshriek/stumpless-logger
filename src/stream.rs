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

use stumpless_sys::*;

use std::error::Error;
use std::ffi::CString;

use crate::error::last_error;
use crate::Target;

pub struct StreamTarget {
    target: *mut stumpless_target,
}

impl StreamTarget {
    pub fn stderr(filename: &str) -> Result<Self, Box<dyn Error>> {
        let c_filename = CString::new(filename)?;
        let stream_target = unsafe { stumpless_open_stderr_target(c_filename.as_ptr()) };

        if stream_target.is_null() {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        } else {
            Ok(StreamTarget {
                target: stream_target,
            })
        }
    }

    pub fn stdout(filename: &str) -> Result<Self, Box<dyn Error>> {
        let c_filename = CString::new(filename)?;
        let stream_target = unsafe { stumpless_open_stdout_target(c_filename.as_ptr()) };

        if stream_target.is_null() {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        } else {
            Ok(StreamTarget {
                target: stream_target,
            })
        }
    }
}

unsafe impl Sync for StreamTarget {}

impl Target for StreamTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

impl Drop for StreamTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_stream_target(self.target);
        }
    }
}
