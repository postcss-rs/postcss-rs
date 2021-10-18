use std::fs;
use std::time::Instant;

use postcss::input::Input;
use postcss::tokenizer::Tokenizer;

fn main() {
    let css = fs::read_to_string("./app.css").unwrap();
    let input = Input::new(css, None);
    let mut processor = Tokenizer::new(input, false);
    while !processor.end_of_file() {
        let start = Instant::now();
        let token = processor.next_token(false);
        let end = start.elapsed();
        println!("{:?}: {:?}", token, end);
    }

    // let input = Input::new("/* hello */\n.cls { font-size: 16px; } .c {".to_string(), None);
    let input = Input::new(":root,[data-color-mode=light][data-light-theme=light],[data-color-mode=dark][data-dark-theme=light]{/*! */}:root,[data-color-mode=light][data-light-theme=light],[data-color-mode=dark][data-dark-theme=light]{--color-canvas-default-transparent: rgba(255,255,255,0);--color-marketing-icon-primary: #218bff;--color-marketing-icon-secondary:".to_string(), None);
    let mut processor = Tokenizer::new(input, true);
    let start = Instant::now();
    println!("{:?}", processor.next_token(false));
    println!("time: {:?}", start.elapsed());

    let start = Instant::now();
    println!("{:?}", processor.next_token(false));
    println!("time: {:?}", start.elapsed());

    let start = Instant::now();
    println!("{:?}", processor.next_token(false));
    println!("time: {:?}", start.elapsed());

    let start = Instant::now();
    println!("{:?}", processor.next_token(false));
    println!("time: {:?}", start.elapsed());

    let start = Instant::now();
    println!("{:?}", processor.next_token(false));
    println!("time: {:?}", start.elapsed());

    let start = Instant::now();
    println!("{:?}", processor.next_token(false));
    println!("time: {:?}", start.elapsed());

    let start = Instant::now();
    println!("{:?}", processor.next_token(false));
    println!("time: {:?}", start.elapsed());

    let start = Instant::now();
    println!("{:?}", processor.next_token(false));
    println!("time: {:?}", start.elapsed());

    let start = Instant::now();
    println!("{:?}", processor.next_token(false));
    println!("time: {:?}", start.elapsed());
}
