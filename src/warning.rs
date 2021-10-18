#[derive(Debug)]
pub struct Warning {
    r#type: String,
    text: String,
    line: u32,
    column: u32,
}

// todo: toString
