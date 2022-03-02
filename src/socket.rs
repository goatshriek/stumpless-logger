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

pub struct SocketTarget {
    target: *mut stumpless_target,
}

impl SocketTarget {
    pub fn new(socket_name: &str) -> Result<Self, Box<dyn Error>> {
        let c_socket_name = CString::new(socket_name)?;
        let socket_target =
            unsafe { stumpless_open_socket_target(c_socket_name.as_ptr(), std::ptr::null()) };

        if socket_target.is_null() {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        } else {
            Ok(SocketTarget {
                target: socket_target,
            })
        }
    }
}

unsafe impl Sync for SocketTarget {}

impl Target for SocketTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

impl Drop for SocketTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_socket_target(self.target);
        }
    }
}
