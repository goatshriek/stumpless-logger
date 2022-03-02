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

pub enum Facility {
    Kernel = stumpless_facility_STUMPLESS_FACILITY_KERN as isize,
    User = stumpless_facility_STUMPLESS_FACILITY_USER as isize,
    Mail = stumpless_facility_STUMPLESS_FACILITY_MAIL as isize,
    Daemon = stumpless_facility_STUMPLESS_FACILITY_DAEMON as isize,
    Auth = stumpless_facility_STUMPLESS_FACILITY_AUTH as isize,
    Syslog = stumpless_facility_STUMPLESS_FACILITY_SYSLOG as isize,
    Lpr = stumpless_facility_STUMPLESS_FACILITY_LPR as isize,
    News = stumpless_facility_STUMPLESS_FACILITY_NEWS as isize,
    Uucp = stumpless_facility_STUMPLESS_FACILITY_UUCP as isize,
    Cron = stumpless_facility_STUMPLESS_FACILITY_CRON as isize,
    Auth2 = stumpless_facility_STUMPLESS_FACILITY_AUTH2 as isize,
    FTP = stumpless_facility_STUMPLESS_FACILITY_FTP as isize,
    NTP = stumpless_facility_STUMPLESS_FACILITY_NTP as isize,
    Audit = stumpless_facility_STUMPLESS_FACILITY_AUDIT as isize,
    Alert = stumpless_facility_STUMPLESS_FACILITY_ALERT as isize,
    Cron2 = stumpless_facility_STUMPLESS_FACILITY_CRON2 as isize,
    Local0 = stumpless_facility_STUMPLESS_FACILITY_LOCAL0 as isize,
    Local1 = stumpless_facility_STUMPLESS_FACILITY_LOCAL1 as isize,
    Local2 = stumpless_facility_STUMPLESS_FACILITY_LOCAL2 as isize,
    Local3 = stumpless_facility_STUMPLESS_FACILITY_LOCAL3 as isize,
    Local4 = stumpless_facility_STUMPLESS_FACILITY_LOCAL4 as isize,
    Local5 = stumpless_facility_STUMPLESS_FACILITY_LOCAL5 as isize,
    Local6 = stumpless_facility_STUMPLESS_FACILITY_LOCAL6 as isize,
    Local7 = stumpless_facility_STUMPLESS_FACILITY_LOCAL7 as isize,
}
