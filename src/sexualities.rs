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
        write!(f, "{:?}", self)
    }
}

impl Sexualities {
    fn get_names(&self) -> Vec<String> {
        match self {
            Self::Gay => vec!["gay", "homosexual"],
            Self::Lesbian => vec!["lesbian", "homosexual"],
            Self::Bi => vec!["bi", "bisexual"],
            Self::Pan => vec!["pan", "pansexual"],
            Self::Ace => vec!["ace", "asexual"],
        }
    }
}
