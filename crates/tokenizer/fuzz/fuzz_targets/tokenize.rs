#![no_main]
use libfuzzer_sys::fuzz_target;

use tokenizer::tokenize;

fuzz_target!(|data: &[u8]| {
    let _ = tokenize(&data);
});