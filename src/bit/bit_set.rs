pub struct BitSet {
    bits: Vec<u32>,
    len: usize,
}

pub struct BitSetIterator<'a> {
    bits: &'a Vec<u32>,
    index: usize,
    len: usize,
}

impl<'a> Iterator for BitSetIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        while self.index < self.len {
            let bit_index = self.index / 32;
            let bit_offset = self.index % 32;
            let bit = self.bits[bit_index] & (1 << bit_offset);
            if bit != 0 {
                let result = self.index;
                self.index += 1;
                return Some(result);
            }
            self.index += 1;
        }
        None // 显式返回 None
    }
}

impl BitSet {
    pub fn iter(&self) -> BitSetIterator {
        BitSetIterator {
            bits: &self.bits,
            index: 0,
            len: self.len,
        }
    }

    pub fn add(&mut self, index: usize) {
        self.bits[index / 32] |= 1 << (index % 32);
    }

    pub fn remove(&mut self, index: usize) {
        self.bits[index / 32] &= !(1 << (index % 32));
    }

    pub fn switch(&mut self, index: usize) {
        self.bits[index / 32] ^= 1 << (index % 32);
    }

    pub fn contains(&self, index: usize) -> bool {
        self.bits[index / 32] & (1 << (index % 32)) != 0
    }

    pub fn clear(&mut self) {
        for i in 0..self.bits.len() {
            self.bits[i] = 0;
        }
    }
}

impl BitSet {
    pub fn new(len: usize) -> BitSet {
        BitSet {
            bits: vec![0; (len + 31) / 32],
            len: len,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bitset() {
        let mut bitset = BitSet::new(100);
        for i in 0..100 {
            bitset.add(i);
        }

        for i in bitset.iter() {
            println!("{}", i);
        }

        for i in bitset.iter() {
            println!("{}", i);
        }

        for i in 0..100 {
            bitset.remove(i);
        }
    }
}
