use crate::SyntaxError;

use std::path::Path;

use extract_line_error::extract_error_code;

pub fn print_error(file: &Path, code: &str, err: SyntaxError) {
    let builder = match err {
        SyntaxError::ExtraToken {
            token: (begin, _, end),
        } => MessageBuilder::new_extra_token()
            .add(get_token(code, begin, end))
            .add(extract_error_code(code, begin, end)),
        SyntaxError::InvalidToken { location } => {
            MessageBuilder::new_invalid_token().add(make_location_message(code, location))
        }
        SyntaxError::UnrecognizedEOF { location, expected } => MessageBuilder::new_unexpected_eof()
            .add(make_location_message(code, location))
            .add(make_expected_value_message(expected)),
        SyntaxError::UnrecognizedToken {
            token: (begin, _, end),
            expected,
        } => MessageBuilder::new_unrecognized_token()
            .add(get_token(code, begin, end))
            .add(extract_error_code(code, begin, end))
            .add(make_expected_value_message(expected)),
        SyntaxError::User { error } => MessageBuilder::new_user_error().add(error),
    };

    print_file_info(file);
    for msg in builder.buff {
        eprintln!("{}", msg);
    }
}

fn print_file_info(file: &Path) {
    let file_name = file.file_name().map(|f| f.to_str().unwrap_or_default());
    if let Some(file_name) = file_name {
        eprintln!("Syntax Error in source file:\n\t{}", file_name);
    } else {
        eprintln!("Syntax Error in [UNKNOWN]  input file");
    }
}

struct MessageBuilder {
    buff: Vec<String>,
}

impl MessageBuilder {
    fn new_extra_token() -> Self {
        Self::new().add("Error: Unexpected Token Found")
    }

    fn new_invalid_token() -> Self {
        Self::new().add("Error: Invalid Token")
    }

    fn new_unexpected_eof() -> Self {
        Self::new()
            .add("Error: Found EOF")
            .add("Found End of File but more code was expected:")
    }

    fn new_unrecognized_token() -> Self {
        Self::new().add("Error: Unrecognized token found")
    }

    fn new_user_error() -> Self {
        Self::new().add("")
    }

    fn new() -> Self {
        Self { buff: vec![] }
    }

    fn add<S: Into<String>>(mut self, msg: S) -> Self {
        let msg = msg.into();
        self.buff.push(msg);
        self
    }
}

fn get_token(code: &str, begin: usize, end: usize) -> String {
    let token = &code[begin..end];
    format!("Token: `{}`", token)
}

fn make_expected_value_message(vals: Vec<String>) -> String {
    let single = vals.len() == 1;
    let vals = vals.join(", ");
    if single {
        format!("Expected: {}", vals)
    } else {
        format!("Expected one of:\n{}", vals)
    }
}

fn make_location_message(code: &str, location: usize) -> String {
    format!("Invalid Token: {}", &code[location..location + 1])
}
