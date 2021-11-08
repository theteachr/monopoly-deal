#[derive(Debug)]
pub struct ColorSet {
    color: Color,
    value: u8,
    size: u8,
    map: Vec<u8>,
}

impl ColorSet {
    pub fn new(color: Color, value: u8, size: u8, map: Vec<u8>) -> Self {
        ColorSet {
            color,
            value,
            size,
            map,
        }
    }
}