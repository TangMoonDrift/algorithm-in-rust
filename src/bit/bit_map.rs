#[derive(Debug, Default)]
pub struct BitMap {
    data: Vec<u64>,
}

pub fn new() -> BitMap {
    BitMap::default()
}
