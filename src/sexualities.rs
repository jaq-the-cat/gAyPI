pub enum Sexualities {
    Gay,
    Lesbian,
    Bi,
    Pan,
    Ace,
}

impl Sexualities {
    pub fn is_valid(sx: &String) -> bool {
        [
            "gay".to_string(),
            "homosexual".to_string(),
            "lesbian".to_string(),
            "homosexual".to_string(),
            "bi".to_string(),
            "bisexual".to_string(),
            "pan".to_string(),
            "pansexual".to_string(),
            "ace".to_string(),
            "asexual".to_string(),
        ]
        .contains(&sx)
    }
}
