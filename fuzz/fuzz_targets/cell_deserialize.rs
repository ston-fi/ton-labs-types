#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut data = data;
    ton_types::deserialize_tree_of_cells(&mut data).ok();
});
