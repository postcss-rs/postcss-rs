use std::fmt;

#[derive(Debug)]
pub struct CssSyntaxError {
    reason: String,
    file: String,
    // source: String,
    plugin: String,
    line: u32,
    column: u32,
}

impl fmt::Display for CssSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}",
            self.plugin, self.file, self.line, self.column, self.reason
        )
    }
}
