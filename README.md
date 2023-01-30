**An enhanced command line logging utility.**

[![crates.io](https://img.shields.io/crates/v/stumpless)](https://crates.io/crates/stumpless)
[![Linux Builds](https://github.com/goatshriek/stumpless-logger/actions/workflows/linux.yml/badge.svg)](https://github.com/goatshriek/stumpless-logger/actions/workflows/linux.yml)
[![Windows Builds](https://github.com/goatshriek/stumpless-logger/actions/workflows/windows.yml/badge.svg)](https://github.com/goatshriek/stumpless-logger/actions/workflows/windows.yml)
[![Mac Builds](https://github.com/goatshriek/stumpless-logger/actions/workflows/mac.yml/badge.svg)](https://github.com/goatshriek/stumpless-logger/actions/workflows/mac.yml)
[![Gitter](https://badges.gitter.im/stumpless/community.svg)](https://gitter.im/stumpless/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)
[![Apache 2.0 License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.1-ff69b4.svg)](https://github.com/goatshriek/stumpless-logger/blob/latest/docs/CODE_OF_CONDUCT.md)

[Key Features](#key-features) |
[Basic Usage](#send-your-logs-anywhere) |
[Contributing](#contributing)


## Key Features
The stumpless logger aims to be a replacement for and improvement over the
traditional `logger` utility. It is written with
[Rust](https://www.rust-lang.org/) and
[Stumpless](https://github.com/goatshriek/stumpless), and offers a number of
improvements over legacy tools, including:

 * more logging target options (files, Windows Event Log)
 * log to multiple destinations with a single invocation
 * portable and tested on Windows and Linux
 * separate thread for each log target


## Send Your Logs Anywhere
The stumpless logger supports all of the target types that Stumpless provides,
which include everything `logger` has and then some.


### The Default Target
Stumpless has the concept of a default target, which attempts to abstract away
the most logical place to send logs on your system. For Windows this is the
Windows Event Log in a log named Stumpless, for Linux and Mac systems this is
/var/run/syslog or /dev/log, and if all else fails this is a file named
`stumpless-default.log`. If you don't provide an explicit target to stumpless,
this is what it will send logs to.

```sh
stumpless Send it!
# where this goes depends on your system!
```

You can explicitly send logs to the default target if you want to, for example
if you need to send to other locations as well as this one, using the
`--default` option like this:

```sh
stumpless --default Send it!
# same as before!
```


#### `stdout` and `stderr`
If you just want to print logs, then use the stdout or stderr.

```sh
stumpless --stdout Hello from Stumpless!
# <13>1 2023-01-01T19:32:19.802953Z dante stumpless-cli - - - Hello from Stumpless!

stumpless --stderr Stumpless says something went wrong...
# <13>1 2023-01-01T19:33:08.957079Z dante stumpless-cli - - - Stumpless says something went wrong...
```


#### Files
Stumpless provides an easy way to write logs to files without going through a
syslog daemon or abusing stream redirection. The `--log-file` flag lets you
specify a file to send logs to, with the predictable choice of `-l` as the short
option.

```sh
stumpless --log-file round.log Everything is a file these days

cat round.log
# <13>1 2023-01-19T02:20:22.984425Z dante stumpless-cli - - - Everything is a file these days
```

If you want to write log entries into more than one file, you can just specify
this flag multiple times. This is true for most log targets in stumpless, and
means that it's simple to send messages to a variety of diverse locations
straight from a shell. Each different log target is handled in its own
thread to prevent them from blocking one another.

```sh
stumpless --log-file square.log --log-file triangle.log You get a message, and you get a message!

# note that the timestamp is slightly different in these two messages
cat square.log
# <13>1 2023-01-22T01:35:07.112856Z dante stumpless-cli - - - You get a message, and you get a message!

cat triangle.log
# <13>1 2023-01-22T01:35:07.112963Z dante stumpless-cli - - - You get a message, and you get a message!
```


#### Network
Sending logs to network servers is a common task. Stumpless supports this with
the `network` feature, which is enabled by default. This supports both IPv4 and
IPv6, for both TCP and UDP. You can specify these with the `--tcp4`, `--udp4`,
`--tcp6`, and `--udp6` options.

```sh
stumpless --tcp4 one-log-server.example Send this message over TCP on IPv4.

stumpless --udp6 two-log-server.example Send this message over UDP on IPv6.

# of course, you can send multiple messages at once, as with other target types:
stumpless --tcp6 red-log-server.example \
          --udp4 blue-log-server.example \
          Send this message to two servers at once!
```

By default, these targets use port 514. If you want to use a different port,
then use the `--port` option (or `-P` short option) to customize this.

```sh
stumpless --tcp4 special-snowflake-1.example --port 7777
          --udp6 special-snowflake-2.exampel --port 8888
          This message goes to two servers on different ports!
```


#### Sockets
If you want to send messages to Unix sockets (such as the traditional
`/dev/log`), then you can use `--socket`, or `-u` for short (think
'u' for Unix). You'll note that this is the same option as `logger` uses.

```sh
stumpless --socket /dev/log Say hello to the daemon for me
```

Socket logging is only available in builds where the `socket` feature has been
enabled.


#### Journald
Of course, stumpless can log to systemd's journaling service if desired. This
uses the same `--journald` option that `logger` users may already be familiar
with.

```sh
stumpless --journald Send this message to the local journald service.
```

Journald logging is only available in builds where the `journald` feature has
been enabled.


#### Windows Event Logs
On machines where a Windows Event Log is present, you can send messages to it as
well. By default this will go to an application log named "Stumpless", but you
change this if you want.

```sh
stumpless --windows-event-log This will go into the Stumpless Application log.

# if you have your own special log, you can send to that instead
stumpless --windows-event-log=MySpecialLog This is a message for my own special log.
```

Note that logs don't show up for applications that aren't configured, including
the default Stumpless log. If you just want to install the default and use it
this way, you can run stumpless with the `--install-wel-default-source` option
to do this. You can run this by itself if you want to install. Note that this
requires enough privileges to make registry changes. It will also point registry
entries it creates at the stumpless executable for resources it needs, so don't
do this until you have stumpless in a place where you plan to leave it.

```sh
# makes registry entries to install the Stumpless application log
# these will point at stumpless.exe, so make sure it's where you want it!
# and that it is in a place that Event Viewer has permissions, if you intend to
# browse the logs through that application
stumpless --install-wel-default-source
```

Windows Event Logging is only available in builds where the `wel` feature has
been enabled.


#### Structured Data
Log entries can often be made easier to parse by using structured data fields.
You can add these with the same options as `logger` uses: `--sd-id` adds an
element, and any `--sd-param` after this adds more detailed fields to the
element. This is easier to understand with some examples:

```sh
# Note that, as with logger, the quotes are required, and may need to be escaped
# for your shell. For bash or cmd.exe this might instead be color=\"red\", for
#PowerShell color='\"red\"', and so on.
stumpless --stdout --sd-id ball --sd-param color="red" --sd-param size="medium" Caught a ball!
# <13>1 2023-01-28T02:34:50.127Z Angus stumpless-cli - - [ball color="red" size="medium"] Caught a ball!

stumpless --stdout \
          --sd-id breadsticks \
          --sd-id mainCourse --sd-param meat="beef" --sd-param side="potatoes" \
          --sd-id dessert --sd-param type="cake" \
          Ate a feast!
# <13>1 2023-01-28T18:53:14.721851Z dante stumpless-cli - - [breadsticks][mainCourse meat="beef" side="potatoes"][dessert type="cake"] Ate a feast!
```


## Differences Between `stumpless` and `logger`
This tool is _not_ written as a drop-in replacement for other `logger`
implementations. This is not to say that it is completely different: most of the
options are the same, and the general modes of use are the same. But there are
differences that arise from decisions made for simplicity, performance, or
necessity. Here are the deviations that are relevant to you if you're already
familiar with or using other loggers.

 * The default output with no arguments is determined by the
   [default target](https://goatshriek.github.io/stumpless/docs/c/latest/target_8h.html#a137ec6ade02951be14bff3572725f076)
   of the underlying stumpless build instead of `/dev/log`.
 * The time quality structured data element is not included (pending Stumpless
   implementation of
   [this feature](https://github.com/goatshriek/stumpless/issues/223)).
 * Network servers IP version and protocol are specified together such as
   `--tcp4` rather than separately via `-T` or `-d` flags independent of the
   `-n` flag. This is to support the specification of multiple targets using
   different combinations in a single invocation.
 * The following flags/modes of operation are not supported:
   * `--rfc3164` for the RFC 3164 BSD syslog format of messages


## Contributing
Notice a problem or have a feature request? Just create an issue using one of
the templates, and we will respond as quickly as we can. You can also look at
the project's [Contribution Guidelines](docs/CONTRIBUTING.md) for more details
on the different ways you can give back to the open source community!

If you want to actually write some code or make an update yourself, there are a
few options based on your level of experience and familiarity with making
contributions.

The first option is to browse the list of issues that are marked with the label
[good first issue](https://github.com/goatshriek/stumpless-logger/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22).
These issues are selected to be a small but meaningful amount of work, and
include details on the general approach that you can take to complete them. They
are a great place to start if you are just looking to test the waters of this
project or open source contribution in general.

More experienced developers may prefer to look at the full list of issues on the
project, as well as the
[roadmap](https://github.com/goatshriek/stumpless-logger/blob/latest/docs/roadmap.md).
If an item catches your interest, drop a comment in the existing issue or open
a new one if it doesn't exist yet and state your intent to work on it so that
others will have a way to know it is underway.

Or perhaps you are just looking for a way to say thanks! If that's the case or
if there is something that you would prefer to drop me a private message about,
please feel free to do so on Twitter with
[#StumplessLib](https://twitter.com/search?q=%23StumplessLib), or in an
[email](mailto:joelanderson333@gmail.com)! I'd love to see you share the project
with others or just hear your thoughts on it.


## Further Documentation
If you're curious about how something in stumpless works that isn't explained
here, you can check the appropriate section of the documentation, stored in the
docs folder of the repository. Folders in the repository contain their own
README files that detail what they contain and any other relevant information.
If you still can't find an answer, submit an issue or head over to
[gitter](https://gitter.im/stumpless/community) and ask for some help.
