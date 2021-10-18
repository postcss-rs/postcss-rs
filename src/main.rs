use postcss::input::Input;
use postcss::tokenizer::Tokenizer;

fn main() {
    // let input = Input::new("/* hello */\n.cls { font-size: 16px; }".to_string(), None);
    let input = Input::new(" \"".to_string(), None);
    let mut processor = Tokenizer::new(input, true);
    println!("{:?}", processor.next_token(false));
    println!("{:?}", processor.next_token(false));
    println!("{:?}", processor.next_token(false));
    println!("{:?}", processor.next_token(false));
    println!("{:?}", processor.next_token(false));
    println!("{:?}", processor.next_token(false));
    println!("{:?}", processor.next_token(false));
}
