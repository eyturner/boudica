#[derive(Debug)]
pub struct Hex {
    pub q: isize,
    pub r: isize,
    pub s: isize,
    pub z: isize,
}

impl Hex {
    pub fn new() -> Hex {
        return Hex {
            q: 0,
            r: 0,
            s: 0,
            z: 0,
        };
    }
}
