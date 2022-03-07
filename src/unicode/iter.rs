use super::{fold::Fold, map::lookup};
use core::array::IntoIter;
use core::str::Chars;

pub struct CanonicalIter<'a> {
    current: Fold,
    chars: Chars<'a>,
}

pub struct CanonicalIterBytes<'a> {
    current: IntoIter<u8, 4>,
    chars: CanonicalIter<'a>,
}

impl<'a> CanonicalIter<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            current: Fold::Zero,
            chars: s.chars(),
        }
    }

    pub fn bytes(self) -> CanonicalIterBytes<'a> {
        let mut empty = IntoIterator::into_iter([0; 4]);
        empty.by_ref().for_each(drop);
        CanonicalIterBytes {
            chars: self,
            current: empty,
        }
    }
}

impl Iterator for CanonicalIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if matches!(self.current, Fold::Zero) {
            self.current = lookup(self.chars.next()?);
        }
        self.current.next()
    }
}

impl Iterator for CanonicalIterBytes<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.as_slice().is_empty() {
            let c = self.chars.chars.next()?;
            let mut bytes = [0; 4];
            let len = c.encode_utf8(&mut bytes).len();
            let mut bytes = IntoIterator::into_iter(bytes);

            assert!(len != 0);

            for _ in len..4 {
                bytes.next_back();
            }

            assert!(!bytes.as_slice().is_empty());

            self.current = bytes
        }
        self.current.next()
    }
}
