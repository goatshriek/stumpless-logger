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

//! Rust bindings for the Stumpless logging library.
//!
//! These wrappings are intended to be a natural-feeling Rust SDK with the same
//! functionality as the underlying Stumpless library. This is different than
//! the stumpless-sys crate, which are raw FFI bindings with no Rust-specific
//! functionality.
//!
//!
//! # Create Features
//! Stumpless provides a number of build configuration options that can be used
//! to enable or disable different functionality. This crate exposes these
//! options as the following features.
//!
//!
//! ### Target Features
//!
//! * **journald** -
//!   Enables targets that can send logs to a systemd journald daemon.
//! * **network** -
//!   Enables targets that can send logs to a server over a network connection.
//! * **socket** -
//!   Enables targets that can send logs to Unix sockets.
//! * **wel** -
//!   Enables targets that can send logs to the Windows Event Log.

use regex::Regex;

mod entry;
pub use crate::entry::Entry;

mod error;
pub use crate::error::{
    invalid_facility_error, invalid_prival_error, invalid_severity_error, perror, StumplessError,
};

mod facility;
pub use crate::facility::Facility;

mod file;
pub use crate::file::FileTarget;

mod severity;
pub use crate::severity::Severity;

mod stream;
pub use crate::stream::StreamTarget;

mod target;
pub use crate::target::{DefaultTarget, Target};

#[cfg(feature = "journald")]
mod journald;
#[cfg(feature = "journald")]
pub use crate::journald::JournaldTarget;

#[cfg(feature = "network")]
mod network;
#[cfg(feature = "network")]
pub use crate::network::NetworkTarget;

#[cfg(feature = "socket")]
mod socket;
#[cfg(feature = "socket")]
pub use crate::socket::SocketTarget;

#[cfg(feature = "wel")]
mod wel;
#[cfg(feature = "wel")]
pub use crate::wel::{add_default_wel_event_source, WelTarget};

// ideally this will become a wrapper for a stumpless native function
// this will give more flexibility to allowed values, and make the logic of the
// cli application simpler
pub fn prival_from_string(priority: &str) -> Result<i32, StumplessError> {
    if let Ok(prival) = priority.parse::<i32>() {
        if (0..=191).contains(&prival) {
            return Ok(prival);
        }
    }

    let priority_re = Regex::new(r"^(\w+).(\w+)$").unwrap();
    match priority_re.captures(priority) {
        Some(caps) => {
            let facility = match caps.get(1).unwrap().as_str() {
                "kern" => 0,
                "user" => 1,
                "mail" => 2,
                "daemon" => 3,
                "auth" | "security" => 4,
                "syslog" => 5,
                "lpr" => 6,
                "news" => 7,
                "uucp" => 8,
                "cron" => 9,
                "authpriv" => 10,
                "ftp" => 11,
                "ntp" => 12,
                "local0" => 16,
                "local1" => 17,
                "local2" => 18,
                "local3" => 19,
                "local4" => 20,
                "local5" => 21,
                "local6" => 22,
                "local7" => 23,
                _ => {
                    return Err(invalid_facility_error());
                }
            };

            let severity = match caps.get(2).unwrap().as_str() {
                "emerg" | "panic" => 0,
                "alert" => 1,
                "crit" => 2,
                "err" | "error" => 3,
                "warning" | "warn" => 4,
                "notice" => 5,
                "info" => 6,
                "debug" => 7,
                _ => {
                    return Err(invalid_severity_error());
                }
            };

            Ok((facility * 8) + severity)
        }
        None => Err(invalid_prival_error()),
    }
}
