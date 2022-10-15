#![no_main]
use libfuzzer_sys::fuzz_target;
use tokenizer::tokenize;

fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data);
    let _ = tokenize(&s);
});