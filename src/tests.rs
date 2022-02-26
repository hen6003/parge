#[cfg(test)]

#[test]
fn long_switch_opt() {
    use crate::parser::*;

    let mut test = false;

    Parser::new()
        .add_switch_opt(
            Some("test"), None,
            &mut test)
        .parse_with(vec!["--test".to_string()]).unwrap();

    assert_eq!(test, true);
}

#[test]
fn long_value_opt() {
    use crate::parser::*;

    let mut test = String::new();

    Parser::new()
        .add_value_opt(
            Some("hello"),
            None,
            &mut test)
        .parse_with(vec!["--hello".to_string(), "world".to_string()]).unwrap();

    assert_eq!(test, "world");
}

#[test]
fn short_switch_opt() {
    use crate::parser::*;

    let mut test = false;

    Parser::new()
        .add_switch_opt(
            None, Some('t'),
            &mut test)
        .parse_with(vec!["-t".to_string()]).unwrap();

    assert_eq!(test, true);
}

#[test]
fn short_value_opt() {
    use crate::parser::*;

    let mut test1 = String::new();
    let mut test2 = String::new();

    Parser::new()
        .add_value_opt(
            None,
            Some('h'),
            &mut test1)
        .add_value_opt(
            None,
            Some('t'),
            &mut test2)
        .parse_with(vec!["-hworld".to_string(), "-t".to_string(), "world".to_string()]).unwrap();

    assert_eq!(test1, "world");
    assert_eq!(test2, "world");
}

#[test]
fn args_after_options() {
    use crate::parser::*;

    let args = Parser::new()
        .parse_with(vec!["--".to_string(), "world".to_string()]);

    assert_eq!(args, Ok(vec!["world".to_string()]));
}

#[test]
fn unknown_option() {
    use crate::parser::*;
    use crate::errors::ParserError;

    let args = Parser::new()
        .parse_with(vec!["--hello".to_string(), "world".to_string()]);

    assert_eq!(args, Err(ParserError::UnknownOption("hello".to_string())));
}
