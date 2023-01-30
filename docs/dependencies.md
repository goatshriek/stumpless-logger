## Build Environment
Almost all of the build environment for the stumpless logger is provided by the
Rust and Cargo toolchains. However, as a result of the dependency on the
`stumpless-sys` crate, which builds the C library, you will also need:
 * `cmake` for build targets and orchestration
 * a toolchain that can be used by cmake (gcc, for example)
