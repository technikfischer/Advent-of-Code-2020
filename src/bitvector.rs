#[derive(Copy, Clone)]
pub struct BitVector {
    lower: u128,
    higher: u128,
}

impl BitVector {
    pub fn new() -> Self {
        Self { lower: 0, higher: 0 }
    }

    pub fn iter(&self) -> BitVectorIterator {
        BitVectorIterator {
            index: 0,
            vector: *self,
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if index < 128 {
            self.lower & (1 << index) > 0
        } else {
            self.higher & (1 << (index - 128)) > 0
        }
    }

    pub fn set(&self, index: usize) -> BitVector {
        if index < 128 {
            BitVector { higher: self.higher, lower: self.lower | (1 << index) }
        } else {
            BitVector { higher: self.higher | (1 << (index - 128)), lower: self.lower }
        }
    }

    pub fn set_mut(&mut self, index: usize) {
        if index < 128 {
            self.lower |= 1 << index;
        } else {
            self.higher |= 1 << (index - 128);
        }
    }
}

pub struct BitVectorIterator {
    index: usize,
    vector: BitVector,
}

/*impl FromIterator<bool> for BitVector {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let mut vec = BitVector::new();
        let mut index = 0;
        for e in iter {
            if e { vec.set_mut(index); }
            index += 1;
        }
        vec
    }
}*/

impl Iterator for BitVectorIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index < 256 {
            Some(self.vector.get(self.index - 1))
        } else {
            None
        }
    }
}