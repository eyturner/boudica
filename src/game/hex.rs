#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
            z: isize::MAX,
        };
    }

    pub fn get_neighbor(&self, dir: HexEdge) -> Hex {
        match dir {
            // N (q, r-1, s + 1, z)
            HexEdge::N => {
                return Hex {
                    q: self.q,
                    r: self.r - 1,
                    s: self.s + 1,
                    z: self.z,
                }
            }
            // NE (q+1, r-1, s, z)
            HexEdge::NE => {
                return Hex {
                    q: self.q + 1,
                    r: self.r - 1,
                    s: self.s,
                    z: self.z,
                }
            }
            // SE (q+1, r, s-1, z)
            HexEdge::SE => {
                return Hex {
                    q: self.q + 1,
                    r: self.r,
                    s: self.s - 1,
                    z: self.z,
                }
            }
            // S (q, r+1, s-1, z)
            HexEdge::S => {
                return Hex {
                    q: self.q,
                    r: self.r + 1,
                    s: self.s - 1,
                    z: self.z,
                }
            }
            // SW (q-1, r+1, s, z)
            HexEdge::SW => {
                return Hex {
                    q: self.q - 1,
                    r: self.r + 1,
                    s: self.s,
                    z: self.z,
                }
            }
            // NW (q-1, r, s+1, z)
            HexEdge::NW => {
                return Hex {
                    q: self.q - 1,
                    r: self.r,
                    s: self.s + 1,
                    z: self.z,
                }
            }
            // T (q, r, s, z+1)
            HexEdge::T => {
                return Hex {
                    q: self.q,
                    r: self.r,
                    s: self.s,
                    z: self.z + 1,
                }
            }
            // B (q, r, s, z-1)
            HexEdge::B => {
                return Hex {
                    q: self.q,
                    r: self.r,
                    s: self.s,
                    z: self.z - 1,
                }
            }
        }
    }

    pub fn get_slide_neighbors(&self) -> Vec<Hex> {
        let mut n = Vec::new();
        n.push(self.get_neighbor(HexEdge::N));
        n.push(self.get_neighbor(HexEdge::NE));
        n.push(self.get_neighbor(HexEdge::SE));
        n.push(self.get_neighbor(HexEdge::S));
        n.push(self.get_neighbor(HexEdge::SW));
        n.push(self.get_neighbor(HexEdge::NW));
        return n;
    }

    pub fn get_neighbors(&self) -> Vec<Hex> {
        let mut n = Vec::new();
        n.push(self.get_neighbor(HexEdge::N));
        n.push(self.get_neighbor(HexEdge::NE));
        n.push(self.get_neighbor(HexEdge::SE));
        n.push(self.get_neighbor(HexEdge::S));
        n.push(self.get_neighbor(HexEdge::SW));
        n.push(self.get_neighbor(HexEdge::NW));
        n.push(self.get_neighbor(HexEdge::T));
        n.push(self.get_neighbor(HexEdge::B));
        return n;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HexEdge {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
    T,
    B,
}

impl HexEdge {
    pub fn get_opposite(self) -> Self {
        match self {
            Self::N => return Self::S,
            Self::NE => return Self::SW,
            Self::SE => return Self::NW,
            Self::S => return Self::N,
            Self::SW => return Self::NE,
            Self::NW => return Self::SE,
            Self::T => return Self::B,
            Self::B => return Self::T,
        }
    }

    pub fn get_gate_edges(self) -> [Self; 2] {
        match self {
            Self::N => return [Self::NW, Self::NE],
            Self::NE => return [Self::N, Self::SE],
            Self::SE => return [Self::NE, Self::S],
            Self::S => return [Self::SE, Self::SW],
            Self::SW => return [Self::S, Self::NW],
            Self::NW => return [Self::SW, Self::N],
            _ => return [Self::N, Self::S], // Impossible
        }
    }
}

pub fn get_edge_types() -> [HexEdge; 8] {
    return [
        HexEdge::N,
        HexEdge::NE,
        HexEdge::SE,
        HexEdge::S,
        HexEdge::SW,
        HexEdge::NW,
        HexEdge::T,
        HexEdge::B,
    ];
}
