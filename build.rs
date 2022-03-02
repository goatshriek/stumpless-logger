#[cfg(feature = "wel")]
use std::path::PathBuf;

#[cfg(feature = "wel")]
use std::env;

#[cfg(feature = "wel")]
use embed_resource;

fn main() {
    #[cfg(feature = "wel")]
    if cfg!(feature = "wel") {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        let mut bin_file = PathBuf::new();
        bin_file.push(&out_dir);
        bin_file.push("default_events_MSG00409.bin");
        stumpless_sys::write_default_events_bin_file(&bin_file).expect("couldn't write bin file");

        let mut resource_file = PathBuf::new();
        resource_file.push(&out_dir);
        resource_file.push("default_events.rc");
        stumpless_sys::write_default_events_resource_file(&resource_file)
            .expect("couldn't write resource file");

        let mut compile_file = PathBuf::new();
        compile_file.push(&out_dir);
        compile_file.push("default_events.rc");
        embed_resource::compile(&compile_file);
    }
}
