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

use stumpless_sys::{stumpless_get_error, stumpless_perror};

use std::error::Error;
use std::ffi::{CStr, CString};
use std::fmt;

#[derive(Debug, Clone)]
pub struct StumplessError {
    //id: i32,
    message: &'static str,
    //code: i32,
    //code_type: &'static str,
}

impl Error for StumplessError {}

impl fmt::Display for StumplessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message)
    }
}

pub fn invalid_facility_error() -> StumplessError {
    StumplessError {
        //id: stumpless_error_id_STUMPLESS_INVALID_FACILITY,
        message: "invalid facility name",
        //code: 0,
        //code_type: "unused",
    }
}

pub fn invalid_prival_error() -> StumplessError {
    StumplessError {
        //id: 1,
        message: "invalid prival format",
        //code: 0,
        //code_type: "unused",
    }
}

pub fn invalid_severity_error() -> StumplessError {
    StumplessError {
        //id: stumpless_error_id_STUMPLESS_INVALID_SEVERITY,
        message: "invalid severity name",
        //code: 0,
        //code_type: "unused",
    }
}

pub fn last_error() -> Result<(), StumplessError> {
    let err = unsafe { stumpless_get_error() };

    if err.is_null() {
        Ok(())
    } else {
        Err(StumplessError {
            //id: unsafe { (*err).id },
            message: unsafe { CStr::from_ptr((*err).message).to_str().unwrap() },
            //code: unsafe { (*err).code },
            //code_type: unsafe { CStr::from_ptr((*err).code_type).to_str().unwrap() },
        })
    }
}

pub fn perror(prefix: &str) {
    let c_prefix = CString::new(prefix).expect("couldn't make a C string");

    unsafe { stumpless_perror(c_prefix.as_ptr()) }
}
