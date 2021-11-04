#![allow(unused_variables)]
#![allow(dead_code)]

use crate::input::Input;
use crate::node::{Node, Root, RootRaws};
use crate::tokenizer::{Token, TokenType, Tokenizer};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser<'a> {
  pub root: Rc<RefCell<Node<'a>>>,
  current: Option<Node<'a>>,
  tokenizer: Tokenizer<'a>,
  spaces: String,
  semicolon: bool,
  custom_property: bool,
}

impl<'a> Parser<'a> {
  pub fn new(input: Input<'a>) -> Self {
    let root = Rc::new(RefCell::new(Node::Root(Root {
      nodes: None,
      parent: None,
      source: None,
      raws: RootRaws::default(),
    })));
    Self {
      root,
      current: None,
      spaces: "".to_string(),
      semicolon: false,
      custom_property: false,
      tokenizer: Tokenizer::new(input.css, true),
    }
  }

  pub fn parse(&mut self) {
    use TokenType::*;
    while !self.tokenizer.end_of_file() {
      let token = self.tokenizer.next_token(true);
      match token.0 {
        Space => self.spaces += &token.1,
        Semicolon => self.free_semicolon(&token),
        CloseCurly => self.end(&token),
        Comment => self.comment(&token),
        AtWord => self.atrule(&token),
        OpenCurly => self.empty_rule(&token),
        _ => self.other(&token),
      }
    }
    self.end_file();
  }

  #[inline]
  fn free_semicolon(&mut self, token: &Token) {
    self.spaces += &token.1;
    if let Some(ref mut node) = self.current {
      if let Node::Rule(ref mut rule) = node {
        if rule.raws.own_semicolon.unwrap_or(false) {
          rule.raws.own_semicolon = Some(!self.spaces.is_empty());
          self.spaces = "".to_owned();
        }
      }
    }
  }

  //   if let Some(node) = self
  //   .current
  //   .nodes_mut()
  //   .and_then(|nodes| nodes.last_mut())
  //   .and_then(|prev| prev.as_any_mut().downcast_mut::<&mut Rule>())
  // {

  #[inline]
  fn end(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn comment(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn atrule(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn empty_rule(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn other(&self, token: &Token) {
    todo!()
  }

  #[inline]
  fn end_file(&self) {
    todo!()
  }
}


// here is postcss-js parser.js source code, port member function one by one

// class Parser {
//   constructor(input) {
//     this.input = input

//     this.root = new Root()
//     this.current = this.root
//     this.spaces = ''
//     this.semicolon = false
//     this.customProperty = false

//     this.createTokenizer()
//     this.root.source = { input, start: { offset: 0, line: 1, column: 1 } }
//   }

//   createTokenizer() {
//     this.tokenizer = tokenizer(this.input)
//   }

//   parse() {
//     let token
//     while (!this.tokenizer.endOfFile()) {
//       token = this.tokenizer.nextToken()

//       switch (token[0]) {
//         case 'space':
//           this.spaces += token[1]
//           break

//         case ';':
//           this.freeSemicolon(token)
//           break

//         case '}':
//           this.end(token)
//           break

//         case 'comment':
//           this.comment(token)
//           break

//         case 'at-word':
//           this.atrule(token)
//           break

//         case '{':
//           this.emptyRule(token)
//           break

//         default:
//           this.other(token)
//           break
//       }
//     }
//     this.endFile()
//   }

//   comment(token) {
//     let node = new Comment()
//     this.init(node, token[2])
//     node.source.end = this.getPosition(token[3] || token[2])

//     let text = token[1].slice(2, -2)
//     if (/^\s*$/.test(text)) {
//       node.text = ''
//       node.raws.left = text
//       node.raws.right = ''
//     } else {
//       let match = text.match(/^(\s*)([^]*\S)(\s*)$/)
//       node.text = match[2]
//       node.raws.left = match[1]
//       node.raws.right = match[3]
//     }
//   }

//   emptyRule(token) {
//     let node = new Rule()
//     this.init(node, token[2])
//     node.selector = ''
//     node.raws.between = ''
//     this.current = node
//   }

//   other(start) {
//     let end = false
//     let type = null
//     let colon = false
//     let bracket = null
//     let brackets = []
//     let customProperty = start[1].startsWith('--')

//     let tokens = []
//     let token = start
//     while (token) {
//       type = token[0]
//       tokens.push(token)

//       if (type === '(' || type === '[') {
//         if (!bracket) bracket = token
//         brackets.push(type === '(' ? ')' : ']')
//       } else if (customProperty && colon && type === '{') {
//         if (!bracket) bracket = token
//         brackets.push('}')
//       } else if (brackets.length === 0) {
//         if (type === ';') {
//           if (colon) {
//             this.decl(tokens, customProperty)
//             return
//           } else {
//             break
//           }
//         } else if (type === '{') {
//           this.rule(tokens)
//           return
//         } else if (type === '}') {
//           this.tokenizer.back(tokens.pop())
//           end = true
//           break
//         } else if (type === ':') {
//           colon = true
//         }
//       } else if (type === brackets[brackets.length - 1]) {
//         brackets.pop()
//         if (brackets.length === 0) bracket = null
//       }

//       token = this.tokenizer.nextToken()
//     }

//     if (this.tokenizer.endOfFile()) end = true
//     if (brackets.length > 0) this.unclosedBracket(bracket)

//     if (end && colon) {
//       while (tokens.length) {
//         token = tokens[tokens.length - 1][0]
//         if (token !== 'space' && token !== 'comment') break
//         this.tokenizer.back(tokens.pop())
//       }
//       this.decl(tokens, customProperty)
//     } else {
//       this.unknownWord(tokens)
//     }
//   }

//   rule(tokens) {
//     tokens.pop()

//     let node = new Rule()
//     this.init(node, tokens[0][2])

//     node.raws.between = this.spacesAndCommentsFromEnd(tokens)
//     this.raw(node, 'selector', tokens)
//     this.current = node
//   }

//   decl(tokens, customProperty) {
//     let node = new Declaration()
//     this.init(node, tokens[0][2])

//     let last = tokens[tokens.length - 1]
//     if (last[0] === ';') {
//       this.semicolon = true
//       tokens.pop()
//     }
//     node.source.end = this.getPosition(last[3] || last[2])

//     while (tokens[0][0] !== 'word') {
//       if (tokens.length === 1) this.unknownWord(tokens)
//       node.raws.before += tokens.shift()[1]
//     }
//     node.source.start = this.getPosition(tokens[0][2])

//     node.prop = ''
//     while (tokens.length) {
//       let type = tokens[0][0]
//       if (type === ':' || type === 'space' || type === 'comment') {
//         break
//       }
//       node.prop += tokens.shift()[1]
//     }

//     node.raws.between = ''

//     let token
//     while (tokens.length) {
//       token = tokens.shift()

//       if (token[0] === ':') {
//         node.raws.between += token[1]
//         break
//       } else {
//         if (token[0] === 'word' && /\w/.test(token[1])) {
//           this.unknownWord([token])
//         }
//         node.raws.between += token[1]
//       }
//     }

//     if (node.prop[0] === '_' || node.prop[0] === '*') {
//       node.raws.before += node.prop[0]
//       node.prop = node.prop.slice(1)
//     }
//     let firstSpaces = this.spacesAndCommentsFromStart(tokens)
//     this.precheckMissedSemicolon(tokens)

//     for (let i = tokens.length - 1; i >= 0; i--) {
//       token = tokens[i]
//       if (token[1].toLowerCase() === '!important') {
//         node.important = true
//         let string = this.stringFrom(tokens, i)
//         string = this.spacesFromEnd(tokens) + string
//         if (string !== ' !important') node.raws.important = string
//         break
//       } else if (token[1].toLowerCase() === 'important') {
//         let cache = tokens.slice(0)
//         let str = ''
//         for (let j = i; j > 0; j--) {
//           let type = cache[j][0]
//           if (str.trim().indexOf('!') === 0 && type !== 'space') {
//             break
//           }
//           str = cache.pop()[1] + str
//         }
//         if (str.trim().indexOf('!') === 0) {
//           node.important = true
//           node.raws.important = str
//           tokens = cache
//         }
//       }

//       if (token[0] !== 'space' && token[0] !== 'comment') {
//         break
//       }
//     }

//     let hasWord = tokens.some(i => i[0] !== 'space' && i[0] !== 'comment')
//     this.raw(node, 'value', tokens)
//     if (hasWord) {
//       node.raws.between += firstSpaces
//     } else {
//       node.value = firstSpaces + node.value
//     }

//     if (node.value.includes(':') && !customProperty) {
//       this.checkMissedSemicolon(tokens)
//     }
//   }

//   atrule(token) {
//     let node = new AtRule()
//     node.name = token[1].slice(1)
//     if (node.name === '') {
//       this.unnamedAtrule(node, token)
//     }
//     this.init(node, token[2])

//     let type
//     let prev
//     let shift
//     let last = false
//     let open = false
//     let params = []
//     let brackets = []

//     while (!this.tokenizer.endOfFile()) {
//       token = this.tokenizer.nextToken()
//       type = token[0]

//       if (type === '(' || type === '[') {
//         brackets.push(type === '(' ? ')' : ']')
//       } else if (type === '{' && brackets.length > 0) {
//         brackets.push('}')
//       } else if (type === brackets[brackets.length - 1]) {
//         brackets.pop()
//       }

//       if (brackets.length === 0) {
//         if (type === ';') {
//           node.source.end = this.getPosition(token[2])
//           this.semicolon = true
//           break
//         } else if (type === '{') {
//           open = true
//           break
//         } else if (type === '}') {
//           if (params.length > 0) {
//             shift = params.length - 1
//             prev = params[shift]
//             while (prev && prev[0] === 'space') {
//               prev = params[--shift]
//             }
//             if (prev) {
//               node.source.end = this.getPosition(prev[3] || prev[2])
//             }
//           }
//           this.end(token)
//           break
//         } else {
//           params.push(token)
//         }
//       } else {
//         params.push(token)
//       }

//       if (this.tokenizer.endOfFile()) {
//         last = true
//         break
//       }
//     }

//     node.raws.between = this.spacesAndCommentsFromEnd(params)
//     if (params.length) {
//       node.raws.afterName = this.spacesAndCommentsFromStart(params)
//       this.raw(node, 'params', params)
//       if (last) {
//         token = params[params.length - 1]
//         node.source.end = this.getPosition(token[3] || token[2])
//         this.spaces = node.raws.between
//         node.raws.between = ''
//       }
//     } else {
//       node.raws.afterName = ''
//       node.params = ''
//     }

//     if (open) {
//       node.nodes = []
//       this.current = node
//     }
//   }

//   end(token) {
//     if (this.current.nodes && this.current.nodes.length) {
//       this.current.raws.semicolon = this.semicolon
//     }
//     this.semicolon = false

//     this.current.raws.after = (this.current.raws.after || '') + this.spaces
//     this.spaces = ''

//     if (this.current.parent) {
//       this.current.source.end = this.getPosition(token[2])
//       this.current = this.current.parent
//     } else {
//       this.unexpectedClose(token)
//     }
//   }

//   endFile() {
//     if (this.current.parent) this.unclosedBlock()
//     if (this.current.nodes && this.current.nodes.length) {
//       this.current.raws.semicolon = this.semicolon
//     }
//     this.current.raws.after = (this.current.raws.after || '') + this.spaces
//   }

//   // Helpers

//   raw(node, prop, tokens) {
//     let token, type
//     let length = tokens.length
//     let value = ''
//     let clean = true
//     let next, prev
//     let pattern = /^([#.|])?(\w)+/i

//     for (let i = 0; i < length; i += 1) {
//       token = tokens[i]
//       type = token[0]

//       if (type === 'comment' && node.type === 'rule') {
//         prev = tokens[i - 1]
//         next = tokens[i + 1]

//         if (
//           prev[0] !== 'space' &&
//           next[0] !== 'space' &&
//           pattern.test(prev[1]) &&
//           pattern.test(next[1])
//         ) {
//           value += token[1]
//         } else {
//           clean = false
//         }

//         continue
//       }

//       if (type === 'comment' || (type === 'space' && i === length - 1)) {
//         clean = false
//       } else {
//         value += token[1]
//       }
//     }
//     if (!clean) {
//       let raw = tokens.reduce((all, i) => all + i[1], '')
//       node.raws[prop] = { value, raw }
//     }
//     node[prop] = value
//   }

//   spacesAndCommentsFromEnd(tokens) {
//     let lastTokenType
//     let spaces = ''
//     while (tokens.length) {
//       lastTokenType = tokens[tokens.length - 1][0]
//       if (lastTokenType !== 'space' && lastTokenType !== 'comment') break
//       spaces = tokens.pop()[1] + spaces
//     }
//     return spaces
//   }

//   spacesAndCommentsFromStart(tokens) {
//     let next
//     let spaces = ''
//     while (tokens.length) {
//       next = tokens[0][0]
//       if (next !== 'space' && next !== 'comment') break
//       spaces += tokens.shift()[1]
//     }
//     return spaces
//   }

//   spacesFromEnd(tokens) {
//     let lastTokenType
//     let spaces = ''
//     while (tokens.length) {
//       lastTokenType = tokens[tokens.length - 1][0]
//       if (lastTokenType !== 'space') break
//       spaces = tokens.pop()[1] + spaces
//     }
//     return spaces
//   }

//   stringFrom(tokens, from) {
//     let result = ''
//     for (let i = from; i < tokens.length; i++) {
//       result += tokens[i][1]
//     }
//     tokens.splice(from, tokens.length - from)
//     return result
//   }

//   colon(tokens) {
//     let brackets = 0
//     let token, type, prev
//     for (let [i, element] of tokens.entries()) {
//       token = element
//       type = token[0]

//       if (type === '(') {
//         brackets += 1
//       }
//       if (type === ')') {
//         brackets -= 1
//       }
//       if (brackets === 0 && type === ':') {
//         if (!prev) {
//           this.doubleColon(token)
//         } else if (prev[0] === 'word' && prev[1] === 'progid') {
//           continue
//         } else {
//           return i
//         }
//       }

//       prev = token
//     }
//     return false
//   }

//   // Errors

//   unclosedBracket(bracket) {
//     throw this.input.error('Unclosed bracket', bracket[2])
//   }

//   unknownWord(tokens) {
//     throw this.input.error('Unknown word', tokens[0][2])
//   }

//   unexpectedClose(token) {
//     throw this.input.error('Unexpected }', token[2])
//   }

//   unclosedBlock() {
//     let pos = this.current.source.start
//     throw this.input.error('Unclosed block', pos.line, pos.column)
//   }

//   doubleColon(token) {
//     throw this.input.error('Double colon', token[2])
//   }

//   unnamedAtrule(node, token) {
//     throw this.input.error('At-rule without name', token[2])
//   }

//   precheckMissedSemicolon(/* tokens */) {
//     // Hook for Safe Parser
//   }

//   checkMissedSemicolon(tokens) {
//     let colon = this.colon(tokens)
//     if (colon === false) return

//     let founded = 0
//     let token
//     for (let j = colon - 1; j >= 0; j--) {
//       token = tokens[j]
//       if (token[0] !== 'space') {
//         founded += 1
//         if (founded === 2) break
//       }
//     }
//     // If the token is a word, e.g. `!important`, `red` or any other valid property's value.
//     // Then we need to return the colon after that word token. [3] is the "end" colon of that word.
//     // And because we need it after that one we do +1 to get the next one.
//     throw this.input.error(
//       'Missed semicolon',
//       token[0] === 'word' ? token[3] + 1 : token[2]
//     )
//   }
// }