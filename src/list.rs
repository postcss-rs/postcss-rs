/// Safely splits values.
pub fn split(input: &str, separators: Vec<char>, last: bool) -> Vec<String> {
    let mut array: Vec<String> = vec![];
    let mut current = String::new();
    let mut split = false;

    let mut func = 0;
    let mut quote = '\0';
    let mut escape = false;

    for letter in input.chars() {
        if escape {
            escape = false;
        } else if letter == '\\' {
            escape = true;
        } else if quote != '\0' {
            if letter == quote {
                quote = '\0';
            }
        } else if letter == '"' || letter == '\'' {
            quote = letter;
        } else if letter == '(' {
            func += 1;
        } else if letter == ')' {
            if func > 0 {
                func -= 1;
            }
        } else if func == 0 && separators.contains(&letter) {
            split = true;
        }

        if split {
            if !current.is_empty() {
                array.push(current.trim().to_string())
            }
            current.clear();
            split = false;
        } else {
            current.push(letter);
        }
    }

    if last || !current.is_empty() {
        array.push(current.trim().to_string());
    }

    array
}

/// Safely splits space-separated values (such as those for `background`,
/// `border-radius`, and other shorthand properties).
pub fn space(input: &str) -> Vec<String> {
    let spaces = vec![' ', '\n', '\t'];
    split(input, spaces, false)
}

/// Safely splits comma-separated values (such as those for `transition-*`
/// and `background` properties).   
pub fn comma(input: &str) -> Vec<String> {
    split(input, vec![','], true)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split() {
        let input = "1px calc(10% + 1px)";
        let s = vec![' ', '\n', '\t'];
        assert_eq!(split(input, s, false), vec!["1px", "calc(10% + 1px)"]);
    }

    #[test]
    fn test_space() {
        let input = "1px calc(10% + 1px)";
        assert_eq!(space(input), vec!["1px", "calc(10% + 1px)"]);
    }

    #[test]
    fn test_comma() {
        let input = "black, linear-gradient(white, black)";
        assert_eq!(comma(input), vec!["black", "linear-gradient(white, black)"]);
    }
}
