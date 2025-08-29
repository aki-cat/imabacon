use imabacon::args;

#[test]
fn parse_invalid_args_returns_err() {
    let invalid_args = ["meow", "foo"].map(|s| String::from(s)).into_iter();
    assert_eq!(
        args::parse(invalid_args).is_err(),
        true,
        "Should have returned Error, but didn't."
    );
}

#[test]
fn parse_invalid_input_err_has_relevant_msg() {
    let invalid_args = ["-o", "foo", "-i"].map(|s| String::from(s)).into_iter();
    match args::parse(invalid_args) {
        Ok(_) => panic!("Should have returned Error, but didn't."),
        Err(msg) => assert_eq!(msg.to_string().contains("invalid input"), true, "{msg}"),
    };
}

#[test]
fn parse_invalid_ouput_err_has_relevant_msg() {
    let invalid_args = ["-i", "foo", "-o"].map(|s| String::from(s)).into_iter();
    match args::parse(invalid_args) {
        Ok(_) => panic!("Should have returned Error, but didn't."),
        Err(msg) => assert_eq!(msg.to_string().contains("invalid output"), true, "{msg}"),
    };
}

#[test]
fn parse_multiple_input_causes_error() {
    let invalid_args = ["-i", "foo", "-i", "-o", "bar"]
        .map(|s| String::from(s))
        .into_iter();
    match args::parse(invalid_args) {
        Ok(_) => panic!("Should have returned Error, but didn't."),
        Err(msg) => assert_eq!(msg.to_string().contains("invalid input"), true, "{msg}"),
    };
}

#[test]
fn parse_multiple_input_clauses_causes_error() {
    let invalid_args = ["-i", "-i", "foo", "-o", "bar"]
        .map(|s| String::from(s))
        .into_iter();
    match args::parse(invalid_args) {
        Ok(_) => panic!("Should have returned Error, but didn't."),
        Err(msg) => assert_eq!(msg.to_string().contains("invalid input"), true),
    };
}

#[test]
fn parse_well_formed_args_pass() {
    let valid_args = ["-i", "meow", "-o", "foo"]
        .map(|s| String::from(s))
        .into_iter();
    assert_eq!(
        args::parse(valid_args).is_ok(),
        true,
        "Valid args failed to parse."
    );
}
