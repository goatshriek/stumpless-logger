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
    stumpless_add_new_element, stumpless_add_new_param_to_entry,
    stumpless_destroy_entry_and_contents, stumpless_entry, stumpless_new_entry_str,
    stumpless_set_entry_prival,
};

use crate::error::last_error;
use crate::facility::Facility;
use crate::severity::Severity;
use std::error::Error;
use std::ffi::CString;

pub struct Entry {
    pub entry: *mut stumpless_entry,
}

impl Entry {
    pub fn new(
        facility: Facility,
        severity: Severity,
        app_name: &str,
        msgid: &str,
        message: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let c_app_name = CString::new(app_name)?;
        let c_msgid = CString::new(msgid)?;
        let c_message = CString::new(message)?;
        let new_entry = unsafe {
            stumpless_new_entry_str(
                (facility as u32).try_into().unwrap(),
                (severity as u32).try_into().unwrap(),
                c_app_name.as_ptr(),
                c_msgid.as_ptr(),
                c_message.as_ptr(),
            )
        };

        if new_entry.is_null() {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        } else {
            Ok(Entry { entry: new_entry })
        }
    }

    pub fn add_new_element(&self, element: &str) -> Result<&Self, Box<dyn Error>> {
        let c_element_name = CString::new(element)?;
        let add_result = unsafe { stumpless_add_new_element(self.entry, c_element_name.as_ptr()) };

        if !add_result.is_null() {
            Ok(self)
        } else {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        }
    }

    pub fn add_new_param(
        &self,
        element: &str,
        param_name: &str,
        param_value: &str,
    ) -> Result<&Self, Box<dyn Error>> {
        let c_element_name = CString::new(element)?;
        let c_param_name = CString::new(param_name)?;
        let c_param_value = CString::new(param_value)?;
        let add_result = unsafe {
            stumpless_add_new_param_to_entry(
                self.entry,
                c_element_name.as_ptr(),
                c_param_name.as_ptr(),
                c_param_value.as_ptr(),
            )
        };

        if !add_result.is_null() {
            Ok(self)
        } else {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        }
    }

    pub fn set_prival(&self, prival: i32) -> Result<&Entry, Box<dyn Error>> {
        let set_result = unsafe { stumpless_set_entry_prival(self.entry, prival) };

        if set_result.is_null() {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        } else {
            Ok(self)
        }
    }
}

impl Drop for Entry {
    fn drop(&mut self) {
        unsafe {
            stumpless_destroy_entry_and_contents(self.entry);
        }
    }
}

unsafe impl Send for Entry {}
unsafe impl Sync for Entry {}
