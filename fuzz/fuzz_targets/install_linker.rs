#![no_main]
use libfuzzer_sys::fuzz_target;
use fleet_rs::utils::configure::install_linker;

fuzz_target!(|linker: String| {
    install_linker(&linker);
});
