use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Idx<T> {
    idx: usize,
    _marker: PhantomData<T>,
}

impl<T> Idx<T> {
    #[inline]
    fn new(idx: usize) -> Self {
        Idx {
            idx: idx,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn index(self) -> usize {
        self.idx
    }
}

pub struct IndexVec<T> {
    raw: Vec<T>,
}

impl<T> IndexVec<T> {
    #[inline]
    pub fn new() -> Self {
        IndexVec { raw: Vec::new() }
    }

    #[inline]
    pub fn push(&mut self, d: T) -> Idx<T> {
        let idx = Idx::new(self.raw.len());
        self.raw.push(d);

        idx
    }

    #[inline]
    pub fn get(&self, idx: Idx<T>) -> &T {
        &self.raw[idx.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn push_and_get() {
        let mut v = IndexVec::new();

        let i0 = v.push(10);
        let i1 = v.push(11);
        let i2 = v.push(12);

        assert_eq!(v.get(i0), &10);
        assert_eq!(v.get(i1), &11);
        assert_eq!(v.get(i2), &12);
    }
}
