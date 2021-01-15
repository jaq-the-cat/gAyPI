pub fn is_valid(sx: &String) -> bool {
    [
        "gay".to_string(),
        "lesbian".to_string(),
        "bi".to_string(),
        "pan".to_string(),
        "ace".to_string(),
    ]
    .contains(&sx)
}
