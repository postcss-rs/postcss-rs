use recursive_parser::{parser::Parser, pretty_print_ast};
fn main() {
  let css = "#id {                     font-size: 12px; 
        width: 100px;
        @media test {
            .test { width: 100px; height: 200px;}
        }
    }";
  let root = Parser::new(css).parse().unwrap();
  let result = pretty_print_ast(&root);
  println!("{}", result);
}
