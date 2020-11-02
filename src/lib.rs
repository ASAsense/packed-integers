use std::marker::PhantomData;

pub mod packed_element;
pub use crate::packed_element::*;

pub struct PackedVec<T: PackedElement> {
    buf: Vec<u32>,
    len: usize,
    phantom: PhantomData<T>,
}

impl<T: PackedElement> PackedVec<T> {
    const U32_NUM_BITS: usize = 32;

    pub fn new() -> PackedVec<T> {
        PackedVec {
            buf: Vec::new(),
            len: 0,
            phantom: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> PackedVec<T> {
        let capacity = (T::NUM_BITS * capacity + (Self::U32_NUM_BITS - 1)) / Self::U32_NUM_BITS;

        PackedVec {
            buf: Vec::with_capacity(capacity),
            len: 0,
            phantom: PhantomData,
        }
    }

    pub fn capacity(&self) -> usize {
        self.buf.capacity() * Self::U32_NUM_BITS / T::NUM_BITS
    }

    pub fn get(&self, index: usize) -> Option<u32> {
        if index >= self.len {
            return None;
        }

        let buf_index = index * T::NUM_BITS / Self::U32_NUM_BITS;
        let start_bit = index * T::NUM_BITS % Self::U32_NUM_BITS;
        let available_bits = Self::U32_NUM_BITS - start_bit;

        if available_bits >= T::NUM_BITS {
            Some((self.buf[buf_index] >> start_bit) & T::MAX)
        } else {
            // Value spans 2 buffer cells.
            let lo = self.buf[buf_index] >> start_bit;
            let hi = self.buf[buf_index + 1] << (Self::U32_NUM_BITS - start_bit);

            Some(lo ^ ((lo ^ hi) & (T::MAX >> available_bits << available_bits)))
        }
    }

    pub fn iter(&self) -> PackedVecIterator<'_, T> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: u32) {
        if value > T::MAX {
            panic!("value is outside the range 0..={}", T::MAX);
        }

        let buf_index = self.len * T::NUM_BITS / Self::U32_NUM_BITS;
        let start_bit = self.len * T::NUM_BITS % Self::U32_NUM_BITS;
        let available_bits = Self::U32_NUM_BITS - start_bit;

        if available_bits >= T::NUM_BITS {
            if buf_index == self.buf.len() {
                self.buf.push(0);
            }

            self.buf[buf_index] |= value << start_bit;
        } else {
            // Value spans 2 buffer cells.
            self.buf.push(0);

            self.buf[buf_index] |= value << start_bit;
            self.buf[buf_index + 1] |= value >> available_bits;
        }

        self.len += 1;
    }
}

pub struct PackedVecIntoIterator<T: PackedElement> {
    vec: PackedVec<T>,
    index: usize,
}

impl<T: PackedElement> IntoIterator for PackedVec<T> {
    type Item = u32;
    type IntoIter = PackedVecIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        PackedVecIntoIterator {
            vec: self,
            index: 0,
        }
    }
}

impl<T: PackedElement> Iterator for PackedVecIntoIterator<T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vec.get(self.index);
        self.index += 1;

        result
    }
}

pub struct PackedVecIterator<'a, T: PackedElement> {
    vec: &'a PackedVec<T>,
    index: usize,
}

impl<'a, T: PackedElement> IntoIterator for &'a PackedVec<T> {
    type Item = u32;
    type IntoIter = PackedVecIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        PackedVecIterator {
            vec: self,
            index: 0,
        }
    }
}

impl<'a, T: PackedElement> Iterator for PackedVecIterator<'a, T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vec.get(self.index);
        self.index += 1;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buflen_no_span() {
        let mut v = PackedVec::<U8>::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        assert_eq!(v.buf.len(), 1);

        v.push(5);
        assert_eq!(v.buf.len(), 2);
    }

    #[test]
    fn buflen_has_span() {
        let mut v = PackedVec::<U9>::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.buf.len(), 1);

        v.push(4);
        assert_eq!(v.buf.len(), 2);
    }

    #[test]
    fn capacity() {
        let v1 = PackedVec::<U9>::with_capacity(7);
        assert_eq!(v1.buf.capacity(), 2);
        assert_eq!(v1.capacity(), 7);

        let v2 = PackedVec::<U9>::with_capacity(8);
        assert_eq!(v2.buf.capacity(), 3);
        assert_eq!(v2.capacity(), 10);
    }
}
