#![no_main]
use libfuzzer_sys::fuzz_target;
use fleet_rs::utils::configure::install_linker;

fuzz_target!(|linker: String| {
    let _ = install_linker(&linker);
});
