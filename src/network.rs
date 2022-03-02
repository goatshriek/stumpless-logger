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

pub struct NetworkTarget {
    target: *mut stumpless_target,
}

impl NetworkTarget {
    pub fn tcp4(server: &str, port: &str) -> Result<Self, Box<dyn Error>> {
        let server_name = CString::new(server)?;
        let network_target = unsafe { stumpless_new_tcp4_target(server_name.as_ptr()) };

        if network_target.is_null() {
            return match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            };
        }

        let tcp_target = NetworkTarget {
            target: network_target,
        };

        tcp_target.set_transport_port(port)?;
        tcp_target.open()?;
        Ok(tcp_target)
    }

    pub fn tcp6(server: &str, port: &str) -> Result<Self, Box<dyn Error>> {
        let server_name = CString::new(server)?;
        let network_target = unsafe { stumpless_new_tcp6_target(server_name.as_ptr()) };

        if network_target.is_null() {
            return match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            };
        }

        let tcp_target = NetworkTarget {
            target: network_target,
        };

        tcp_target.set_transport_port(port)?;
        tcp_target.open()?;
        Ok(tcp_target)
    }

    pub fn udp4(server: &str, port: &str) -> Result<Self, Box<dyn Error>> {
        let server_name = CString::new(server)?;
        let network_target = unsafe { stumpless_new_udp4_target(server_name.as_ptr()) };

        if network_target.is_null() {
            return match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            };
        }

        let udp_target = NetworkTarget {
            target: network_target,
        };

        udp_target.set_transport_port(port)?;
        udp_target.open()?;
        Ok(udp_target)
    }

    pub fn udp6(server: &str, port: &str) -> Result<Self, Box<dyn Error>> {
        let server_name = CString::new(server)?;
        let network_target = unsafe { stumpless_new_udp6_target(server_name.as_ptr()) };

        if network_target.is_null() {
            return match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            };
        }

        let udp_target = NetworkTarget {
            target: network_target,
        };

        udp_target.set_transport_port(port)?;
        udp_target.open()?;
        Ok(udp_target)
    }

    fn set_transport_port(&self, port: &str) -> Result<(), Box<dyn Error>> {
        let port_name = CString::new(port)?;
        let port_result = unsafe { stumpless_set_transport_port(self.target, port_name.as_ptr()) };
        if port_result.is_null() {
            match last_error() {
                Ok(_success) => panic!("inconsistent stumpless error state"),
                Err(err) => Err(Box::new(err)),
            }
        } else {
            Ok(())
        }
    }
}

unsafe impl Sync for NetworkTarget {}

impl Target for NetworkTarget {
    fn get_pointer(&self) -> *mut stumpless_target {
        self.target
    }
}

impl Drop for NetworkTarget {
    fn drop(&mut self) {
        unsafe {
            stumpless_close_network_target(self.target);
        }
    }
}
