#[cfg(not(target_os = "windows"))]
include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));
