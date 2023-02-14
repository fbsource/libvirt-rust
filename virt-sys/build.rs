use std::error::Error;
use std::path::PathBuf;
use std::{env, process};

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=wrapper.h");
    let mut config = pkg_config::Config::new();

    let mut bindings = bindgen::builder()
        .header("wrapper.h")
        .allowlist_var("^(VIR_|vir).*")
        .allowlist_type("^vir.*")
        .allowlist_function("^vir.*")
        // this is only false on esoteric platforms which libvirt does not support
        .size_t_is_usize(true)
        .generate_comments(false)
        .prepend_enum_name(false)
        .ctypes_prefix("::libc");

    config
        .atleast_version(if cfg!(feature = "libvirt-7-5-0") {
            "7.5.0"
        } else if cfg!(feature = "libvirt-7-1-0") {
            "7.1.0"
        } else if cfg!(feature = "libvirt-6-10-0") {
            "6.10.0"
        } else if cfg!(feature = "libvirt-6-1-0") {
            "6.1.0"
        } else if cfg!(feature = "libvirt-5-10-0") {
            "5.10.0"
        } else {
            "5.7.0"
        })
        .probe("libvirt")?;

    if cfg!(feature = "qemu") {
        config.probe("libvirt-qemu")?;

        bindings = bindings
            .clang_arg("-DBINDGEN_USE_QEMU");
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    bindings
        .generate()
        .map_err(|_| String::from("could not generate bindings"))?
        .write_to_file(out_dir.join("bindings.rs"))?;

    Ok(())
}
