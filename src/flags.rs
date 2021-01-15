use crate::map;
use serde::Serialize;
use std::collections::HashMap;

#[macro_export]
macro_rules! flags {
    ($flag: expr) => {{
        match $flag {
            "gay" => [
                "#E40203", "#FF8B00", "#FEED00", "#008026", "#004DFF", "#750686",
            ],
            "lesbian" => ["#D52D00", "#FF9A56", "#FFFFFF", "#D362A4", "#A30262"],
            "bi" => ["#D80271", "#D80271", "#734F95", "#0038A7", "#0038A7"],
            "pan" => ["#FF228C", "#FFD900", "#1BB3FF"],
            "ace" => ["#000000", "#A2A2A2", "#FFFFFF", "#80007E"],
            "trans" => ["#5AC9F4", "#EFA4B3", "#FBFBFB", "#EFA4B3", "#5AC9F4"],
            _ => [],
        }
    }};
}

#[derive(Serialize)]
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
