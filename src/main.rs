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

use clap::{command, crate_version, parser::ValueSource, value_parser, Arg, ArgAction};
use itertools::Itertools;
use regex::Regex;
use std::{
    sync::Arc,
    thread::{spawn, JoinHandle},
};
use stumpless::{
    perror, prival_from_string, DefaultTarget, Entry, Facility, FileTarget, Severity, StreamTarget,
    Target,
};

#[cfg(feature = "journald")]
use stumpless::JournaldTarget;

#[cfg(feature = "network")]
use stumpless::NetworkTarget;

#[cfg(feature = "socket")]
use stumpless::SocketTarget;

#[cfg(feature = "wel")]
use stumpless::{add_default_wel_event_source, WelTarget};

fn main() {
    let default_long_help = "\
        If no other targets are specified, then the default target will be \
        used. You can explicitly ask for logs to be sent to the default target \
        as well others by specifying this option.\
        \n\nThe default target depends on the build of stumpless used. \
        Generally, if Windows Event Log targets are supported, then the \
        default target will be an event log named Stumpless. If Windows Event \
        Log targets are not supported and socket targets are, then the default \
        target will be the socket named either /var/run/syslog or /dev/log. If \
        neither of these target types are supported then logs are written to a \
        file target is opened to log to a file named stumpless-default.log.\
        \n\nConsult the stumpless documentation for \
        stumpless_get_default_target for the nuances of the default target.";
    let default_arg = Arg::new("default")
        .long("default")
        .help("Log to the default log target.")
        .long_help(default_long_help)
        .required(false);

    let file_arg = Arg::new("file")
        .short('f')
        .long("file")
        .help("Log the contents of the file instead of reading from stdin or message arg.")
        .required(false);

    let id_long_help = "\
        When the optional argument id is specified, then it is used instead of \
        the executable's PID. It's recommended to set this to a single value \
        in scripts that send multiple messages, for example the script's own \
        process id.\
        \n\n\
        Note that some logging infrastructure (for example systemd when \
        listening on /dev/log) may overwrite this value, for example with the \
        one derived from the connecting socket.";
    let id_arg = Arg::new("id")
        .short('i')
        .long("id")
        .value_name("id")
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("todo")
        .help("Log a PID in each entry. Defaults to the PID of the CLI process.")
        .long_help(id_long_help);

    let journald_arg = Arg::new("journald")
        .short('j')
        .long("journald")
        .help("Log the entry to the journald system.")
        .required(false)
        .num_args(0..=1)
        .require_equals(true);

    let log_file_long_help = "\
        This option can be provided as many times as needed with different files
        to log to multiple files with one invocation.";
    let log_file_arg = Arg::new("log-file")
        .short('l')
        .long("log-file")
        .value_name("file")
        .help("Log the entry to the given file.")
        .long_help(log_file_long_help)
        .required(false)
        .action(ArgAction::Append);

    let message_arg = Arg::new("message")
        .help("The message to send in the log entry.")
        .num_args(1..)
        .required_unless_present("install-wel-default-source");

    let msgid_arg = Arg::new("msgid")
        .short('m')
        .long("msgid")
        .help("The msgid to use in the message.")
        .default_value("-")
        .required(false);

    let priority_long_help = "\
        The priority may be specified as an integer, in which case it must be \
        defined as what is specified in RFC 5424 as the prival.\
        \n\n\
        This may also be provided in a human readable format of \
        <facility>.<level>. Capitalization is ignored.\
        \n\nSeverity levels:\n\
        emerg or panic\n\
        alert\n\
        crit\n\
        err or error\n\
        warning or warn\n\
        notice\n\
        info\n\
        debug\n\
        \n\nFacility levels:\n\
        kern               kernel messages\n\
        user               user-level messages\n\
        mail               mail system facility code value\n\
        daemon             system daemons\n\
        auth or security   security/authorization messages\n\
        syslog             message generated by the logging daemon\n\
        lpr                line printer subsystem\n\
        news               network news\n\
        uucp               uucp subsystem\n\
        cron               clock daemon\n\
        auth2              security/authorization messages\n\
        ftp                ftp daemon\n\
        ntp                ntp subsystem\n\
        audit              log audit\n\
        alert              log alert\n\
        cron2              clock daemon\n\
        local0             local use 0\n\
        local1             local use 1\n\
        local2             local use 2\n\
        local3             local use 3\n\
        local4             local use 4\n\
        local5             local use 5\n\
        local6             local use 6\n\
        local7             local use 7";
    let priority_arg = Arg::new("priority")
        .short('p')
        .long("priority")
        .value_name("priority")
        .help("The priority of the message to be sent.")
        .long_help(priority_long_help)
        .required(false);

    let sd_id_arg = Arg::new("sd-id")
        .long("sd-id")
        .value_name("name")
        .value_parser(value_parser!(String))
        .help("Include the structured data element id.")
        .required(false)
        .action(ArgAction::Append);

    let sd_param_arg = Arg::new("sd-param")
        .long("sd-param")
        .value_name("name=\"value\"")
        .value_parser(value_parser!(String))
        .help("Add a parameter name and value to the previous element id.")
        .required(false)
        .action(ArgAction::Append);

    let socket_arg = Arg::new("socket")
        .short('u')
        .long("socket")
        .value_name("socket")
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("/dev/log")
        .help("Log to the provided socket, defaulting to /dev/log.")
        .required(false)
        .action(ArgAction::Append);

    let stderr_arg = Arg::new("stderr")
        .short('s')
        .long("stderr")
        .action(ArgAction::SetTrue)
        .help("Log to stderr.")
        .required(false);

    let stdout_arg = Arg::new("stdout")
        .long("stdout")
        .action(ArgAction::SetTrue)
        .help("Log to stdout.")
        .required(false);

    let tcp4_arg = Arg::new("tcp4")
        .short('T')
        .long("tcp4")
        .value_name("server")
        .help("Send the entry to the given server using TCP over IPv4.")
        .required(false)
        .action(ArgAction::Append);

    let tcp6_arg = Arg::new("tcp6")
        .long("tcp6")
        .value_name("server")
        .help("Send the entry to the given server using TCP over IPv6.")
        .required(false)
        .action(ArgAction::Append);

    let udp4_arg = Arg::new("udp4")
        .short('d')
        .long("udp4")
        .value_name("server")
        .help("Send the entry to the given server using UDP over IPv4.")
        .required(false)
        .action(ArgAction::Append);

    let udp6_arg = Arg::new("udp6")
        .long("udp6")
        .value_name("server")
        .help("Send the entry to the given server using UDP over IPv6.")
        .required(false)
        .action(ArgAction::Append);

    let wel_arg = Arg::new("windows-event-log")
        .short('w')
        .long("windows-event-log")
        .value_name("log")
        .help("Log to the Windows Event Log provided, defaulting to Stumpless.")
        .num_args(0..=1)
        .default_missing_value("Stumpless")
        .require_equals(true)
        .required(false)
        .action(ArgAction::Append);

    let wel_install_long_help = "\
        Having the event source information installed is required for the \
        Event Viewer to properly display events logged to it. This only needs \
        to happen once, and can be done after the events themselves are logged \
        with no loss of information. This option requires privileges to access \
        and modify the Windows Registry to function properly. The created \
        registry entries will point at the stumpless executable, so having it \
        in a location with restricted privileges or moving it after running \
        this may break some log visibility.";
    let wel_install_arg = Arg::new("install-wel-default-source")
        .long("install-wel-default-source")
        .help("Installs the stumpless default Windows Event Log source.")
        .long_help(wel_install_long_help)
        .num_args(0)
        .required(false);

    let cli_matches = command!()
        .version(crate_version!())
        .arg(default_arg)
        .arg(file_arg)
        .arg(id_arg)
        .arg(journald_arg)
        .arg(log_file_arg)
        .arg(message_arg)
        .arg(msgid_arg)
        .arg(priority_arg)
        .arg(sd_id_arg)
        .arg(sd_param_arg)
        .arg(socket_arg)
        .arg(stderr_arg)
        .arg(stdout_arg)
        .arg(tcp4_arg)
        .arg(tcp6_arg)
        .arg(udp4_arg)
        .arg(udp6_arg)
        .arg(wel_arg)
        .arg(wel_install_arg)
        .get_matches();

    #[cfg(feature = "wel")]
    if cli_matches.value_source("install-wel-default-source") == Some(ValueSource::CommandLine) {
        add_default_wel_event_source()
            .expect("adding the default Windows Event Log source failed!");
    }

    #[cfg(not(feature = "wel"))]
    if cli_matches.value_source("install-wel-default-source") == Some(ValueSource::CommandLine) {
        eprintln!("Windows Event Log functionality is not enabled, ignoring --install-wel-default-source option")
    }

    if !cli_matches.contains_id("message") {
        // we are all done if there is no message to log
        println!("exiting with no message");
        return;
    }

    let message_iterator = cli_matches
        .get_many::<String>("message")
        .unwrap()
        .map(|s| s.as_str());
    let message = Itertools::intersperse(message_iterator, " ").collect::<String>();

    let entry = Entry::new(
        Facility::User,
        Severity::Notice,
        "stumpless-cli",
        cli_matches.get_one::<String>("msgid").unwrap(),
        &message,
    )
    .expect("entry creation failed!");

    // build the elements and param structured data entries
    let element_indices: Vec<usize> = match cli_matches.indices_of("sd-id") {
        Some(index_iterator) => index_iterator.collect(),
        None => Vec::new(),
    };
    let elements: Vec<&str> = match cli_matches.get_many::<String>("sd-id") {
        Some(element_iterator) => element_iterator.map(|s| s.as_str()).collect(),
        None => Vec::new(),
    };
    assert_eq!(element_indices.len(), elements.len());

    let param_indices: Vec<usize> = match cli_matches.indices_of("sd-param") {
        Some(index_iterator) => index_iterator.collect(),
        None => Vec::new(),
    };
    let params: Vec<&str> = match cli_matches.get_many::<String>("sd-param") {
        Some(param_iterator) => param_iterator.map(|s| s.as_str()).collect(),
        None => Vec::new(),
    };
    assert_eq!(param_indices.len(), params.len());

    let mut param_i: usize = 0;
    let param_regex = Regex::new(r#"(.*)="(.*)""#).expect("the param regex failed to compile");
    for element_i in 0..elements.len() {
        entry
            .add_new_element(elements[element_i])
            .expect("couldn't add an element to the entry");

        while param_i < params.len()
            && param_indices[param_i] > element_indices[element_i]
            && (elements.len() == element_i + 1
                || param_indices[param_i] < element_indices[element_i + 1])
        {
            let param_captures = param_regex
                .captures(params[param_i])
                .expect("provided param value was not formatted correctly");
            let param_name = param_captures.get(1).map_or("", |m| m.as_str());
            let param_value = param_captures.get(2).map_or("", |m| m.as_str());
            entry
                .add_new_param(elements[element_i], param_name, param_value)
                .expect("couldn't add a new param to the entry");
            param_i += 1;
        }
    }

    if cli_matches.contains_id("priority") {
        let priority = cli_matches.get_one::<String>("priority").unwrap();
        let prival = prival_from_string(priority).expect("could not parse priority");
        entry.set_prival(prival).expect("priority invalid");
    }

    let mut log_threads: Vec<JoinHandle<()>> = Vec::with_capacity(64); // arbitrary size
    let mut default_needed = true;
    let entry_arc = Arc::new(entry);

    if let Some(true) = cli_matches.get_one::<bool>("stderr") {
        default_needed = false;
        let entry_clone = Arc::clone(&entry_arc);
        log_threads.push(spawn(move || {
            let stderr_target = StreamTarget::stderr("stderr").unwrap();
            stderr_target
                .add_entry(&entry_clone)
                .expect("logging to stderr failed!");
        }));
    }

    if let Some(true) = cli_matches.get_one::<bool>("stdout") {
        default_needed = false;
        let entry_clone = Arc::clone(&entry_arc);
        log_threads.push(spawn(move || {
            let stdout_target = StreamTarget::stdout("stdout").unwrap();
            stdout_target
                .add_entry(&entry_clone)
                .expect("logging to stdout failed!");
        }));
    }

    if let Some(log_files) = cli_matches.get_many::<String>("log-file") {
        for log_file in log_files {
            default_needed = false;
            let log_filename = log_file.clone();
            let entry_clone = Arc::clone(&entry_arc);
            log_threads.push(spawn(move || {
                match FileTarget::new(&log_filename) {
                    Err(_error) => perror("opening the file target failed"),
                    Ok(target) => {
                        if let Err(_error) = target.add_entry(&entry_clone) {
                            perror("logging to the file target failed");
                        }
                    }
                };
            }));
        }
    }

    #[cfg(feature = "journald")]
    if cli_matches.contains_id("journald") {
        default_needed = false;
        let entry_clone = Arc::clone(&entry_arc);
        log_threads.push(spawn(move || {
            let journald_target = JournaldTarget::new().unwrap();
            journald_target
                .add_entry(&entry_clone)
                .expect("logging to journald failed!");
        }));
    }

    #[cfg(not(feature = "journald"))]
    if cli_matches.contains_id("journald") {
        eprintln!("journald logging not enabled, ignoring --journald option");
    }

    #[cfg(feature = "socket")]
    if let Some(sockets) = cli_matches.get_many::<String>("socket") {
        for socket in sockets {
            default_needed = false;
            let entry_clone = Arc::clone(&entry_arc);
            let socket_name = socket.clone();
            log_threads.push(spawn(move || {
                let socket_target = SocketTarget::new(&socket_name).unwrap();
                socket_target
                    .add_entry(&entry_clone)
                    .expect("logging to socket failed!");
            }));
        }
    }

    #[cfg(not(feature = "socket"))]
    if cli_matches.contains_id("socket") {
        eprintln!("socket logging not enabled, ignoring --socket option");
    }

    #[cfg(feature = "network")]
    if let Some(servers) = cli_matches.get_many::<String>("tcp4") {
        for server in servers {
            default_needed = false;
            let entry_clone = Arc::clone(&entry_arc);
            let server_name = server.clone();
            log_threads.push(spawn(move || {
                let tcp4_target = NetworkTarget::tcp4(&server_name, "514").unwrap();
                tcp4_target
                    .add_entry(&entry_clone)
                    .expect("logging to tcp4 failed");
            }));
        }
    }

    #[cfg(not(feature = "network"))]
    if cli_matches.contains_id("tcp4") {
        eprintln!("network logging not enabled, ignoring --tcp4 option");
    }

    #[cfg(feature = "network")]
    if let Some(servers) = cli_matches.get_many::<String>("tcp6") {
        for server in servers {
            default_needed = false;
            let entry_clone = Arc::clone(&entry_arc);
            let server_name = server.clone();
            log_threads.push(spawn(move || {
                let tcp6_target = NetworkTarget::tcp6(&server_name, "514").unwrap();
                tcp6_target
                    .add_entry(&entry_clone)
                    .expect("logging to tcp6 failed");
            }));
        }
    }

    #[cfg(not(feature = "network"))]
    if cli_matches.contains_id("tcp6") {
        eprintln!("network logging not enabled, ignoring --tcp6 option");
    }

    #[cfg(feature = "network")]
    if let Some(servers) = cli_matches.get_many::<String>("udp4") {
        for server in servers {
            default_needed = false;
            let entry_clone = Arc::clone(&entry_arc);
            let server_name = server.clone();
            log_threads.push(spawn(move || {
                let udp4_target = NetworkTarget::udp4(&server_name, "514").unwrap();
                udp4_target
                    .add_entry(&entry_clone)
                    .expect("logging to udp4 failed");
            }));
        }
    }

    #[cfg(not(feature = "network"))]
    if cli_matches.contains_id("udp4") {
        eprintln!("network logging not enabled, ignoring --udp4 option");
    }

    #[cfg(feature = "network")]
    if let Some(servers) = cli_matches.get_many::<String>("udp6") {
        for server in servers {
            default_needed = false;
            let entry_clone = Arc::clone(&entry_arc);
            let server_name = server.clone();
            log_threads.push(spawn(move || {
                let udp6_target = NetworkTarget::tcp6(&server_name, "514").unwrap();
                udp6_target
                    .add_entry(&entry_clone)
                    .expect("logging to udp6 failed");
            }));
        }
    }

    #[cfg(not(feature = "network"))]
    if cli_matches.contains_id("udp6") {
        eprintln!("network logging not enabled, ignoring --udp6 option");
    }

    #[cfg(feature = "wel")]
    if cli_matches.value_source("windows-event-log") == Some(ValueSource::CommandLine) {
        if let Some(wel_logs) = cli_matches.get_many::<String>("windows-event-log") {
            for wel_log in wel_logs {
                default_needed = false;
                let entry_clone = Arc::clone(&entry_arc);
                let wel_log_name = wel_log.clone();
                log_threads.push(spawn(move || {
                    let wel_target = WelTarget::new(&wel_log_name).unwrap();
                    wel_target
                        .add_entry(&entry_clone)
                        .expect("logging to the Windows Event Log failed!");
                }));
            }
        }
    }

    #[cfg(not(feature = "wel"))]
    if cli_matches.value_source("windows-event-log") == Some(ValueSource::CommandLine) {
        eprintln!("Windows Event Log logging is not enabled, ignoring --windows-event-log option");
    }

    if cli_matches.contains_id("default") || default_needed {
        let entry_clone = Arc::clone(&entry_arc);
        log_threads.push(spawn(move || {
            let default_target = DefaultTarget::get_default_target().unwrap();
            default_target
                .add_entry(&entry_clone)
                .expect("logging to the default target failed!");
        }));
    }

    for handle in log_threads {
        handle
            .join()
            .expect("Couldn't join one of the logging threads!");
    }
}
