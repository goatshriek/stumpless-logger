# Changelog
All notable changes to the stumpless logger will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.1.1] - 2023-11-10
### Security
 - Addressed Github Security Advisory GHSA-c827-hfw6-qwvm.


## [0.1.0] - 2023-01-28
### Added
 - Logging to stdout, stderr, files, journald, network, socket, and Windows
   Event Log endpoints.
 - `journald`, `network`, `socket`, and `wel` features based on the target types
   within stumpless-sys.
