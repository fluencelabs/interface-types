pub struct ByteAccess<'a> {
    pub slice: MemSlice2<'a>,
    pub index: usize,
}

impl ByteAccess<'_> {
    pub fn get(&self) -> u8 {
        self.slice.get(self.index)
    }
    pub fn set(&self, val: u8) {
        self.slice.set(self.index, val)
    }
}
use std::cell::Cell;
use std::ptr::slice_from_raw_parts;

pub trait MemSlice3 {
    fn len(&self) -> usize;
    fn index(&self, index: usize) -> ByteAccess;
    fn get(&self, index: usize) -> u8;
    fn set(&self, index: usize, value: u8);
    fn range_iter(&self, begin: usize, end: usize) -> MemSliceIter;
}

#[derive(Clone, Copy)]
pub struct MemSlice2<'a> {
    pub slice_ref: &'a dyn MemSlice3,
}
impl MemSlice2<'_> {
    pub fn len(&self) -> usize {
        self.slice_ref.len()
    }

    pub fn index(&self, index: usize) -> ByteAccess {
        self.slice_ref.index(index)
    }

    pub fn get(&self, index: usize) -> u8 {
        self.slice_ref.get(index)
    }

    pub fn set(&self, index: usize, value: u8) {
        self.slice_ref.set(index, value)
    }

    pub fn range_iter(&self, begin: usize, end: usize) -> MemSliceIter {
        self.slice_ref.range_iter(begin, end)
    }
}

pub struct MemSliceIter<'a> {
    pub begin: usize,
    pub end: usize,
    pub slice: MemSlice2<'a>,
}

impl<'a> MemSliceIter<'a> {
    pub fn new(begin: usize, end: usize, slice: MemSlice2<'a>) -> Self {
        Self { begin, end, slice }
    }
}

impl<'a> Iterator for MemSliceIter<'a> {
    type Item = ByteAccess<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin < self.end {
            let res = Some(ByteAccess {
                slice: self.slice,
                index: self.begin,
            });
            self.begin += 1;
            res
        } else {
            None
        }
    }
}
