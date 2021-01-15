#[derive(Clone)]
pub struct RGB(u8, u8, u8);

pub struct PrideFlag {
    name: &'static str,
    colors: Vec<RGB>,
    stripes: usize,
}

impl PrideFlag {
    pub fn new(name: &'static str, colors: Vec<RGB>) -> Self {
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
            RGB(230, 0, 0),
            RGB(254, 138, 1),
            RGB(251, 234, 2),
            RGB(1, 125, 34),
            RGB(0, 50, 255),
            RGB(116, 5, 131),
        ],
    )
}
