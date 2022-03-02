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

pub enum Severity {
    Emergency = stumpless_severity_STUMPLESS_SEVERITY_EMERG as isize,
    Alert = stumpless_severity_STUMPLESS_SEVERITY_ALERT as isize,
    Critical = stumpless_severity_STUMPLESS_SEVERITY_CRIT as isize,
    Error = stumpless_severity_STUMPLESS_SEVERITY_ERR as isize,
    Warning = stumpless_severity_STUMPLESS_SEVERITY_WARNING as isize,
    Notice = stumpless_severity_STUMPLESS_SEVERITY_NOTICE as isize,
    Info = stumpless_severity_STUMPLESS_SEVERITY_INFO as isize,
    Debug = stumpless_severity_STUMPLESS_SEVERITY_DEBUG as isize,
}
