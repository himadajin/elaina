#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SpanData {
    pub lo: u32,
    pub hi: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Span {
    index: u32,
    len: u16,
}

const MAX_LEN: u32 = 0b0111_1111_1111_1111;

pub const DUMMY_SP: Span = Span { index: 0, len: 0 };

impl Span {
    #[inline]
    pub fn new(mut lo: u32, mut hi: u32) -> Self {
        if lo > hi {
            std::mem::swap(&mut lo, &mut hi);
        }

        let (base, len) = (lo, hi - lo);

        if len <= MAX_LEN {
            Span {
                index: base,
                len: len as u16,
            }
        } else {
            panic!("too long span");
        }
    }

    #[inline]
    pub fn data(self) -> SpanData {
        SpanData {
            lo: self.index,
            hi: self.index + self.len as u32,
        }
    }
}
