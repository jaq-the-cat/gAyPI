pub struct PrideFlag {
    name: &'static str,
    colors: Vec<&'static str>,
    stripes: usize,
}

impl PrideFlag {
    pub fn new(name: &'static str, colors: Vec<&'static str>) -> Self {
        let len = colors.len();
        Self {
            name,
            colors,
            stripes: len,
        }
    }
}

pub fn gay() -> PrideFlag {
    PrideFlag::new(
        "gay",
        vec![
            "#E40203", "#FF8B00", "#FEED00", "#008026", "#004DFF", "#750686",
        ],
    )
}

pub fn lesbian() -> PrideFlag {
    PrideFlag::new(
        "lesbian",
        vec!["#D52D00", "#FF9A56", "#FFFFFF", "#D362A4", "#A30262"],
    )
}

pub fn bi() -> PrideFlag {
    PrideFlag::new(
        "bi",
        vec!["#D80271", "#D80271", "#734F95", "#0038A7", "#0038A7"],
    )
}

pub fn pan() -> PrideFlag {
    PrideFlag::new("pan", vec!["#FF228C", "#FFD900", "#1BB3FF"])
}

pub fn ace() -> PrideFlag {
    PrideFlag::new("ace", vec!["#000000", "#A2A2A2", "#FFFFFF", "#80007E"])
}
