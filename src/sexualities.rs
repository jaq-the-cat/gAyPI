use std::fmt;

enum Sexualities {
    Gay,
    Lesbian,
    Bi,
    Pan,
    Ace,
}

impl fmt::Display for Sexualities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
