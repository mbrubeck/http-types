use std::collections::hash_map;
use std::iter::Iterator;

use crate::headers::{HeaderName, HeaderValue};

/// An owning iterator over the entries of `Headers`.
#[derive(Debug)]
pub struct IntoIter {
    pub(super) inner: hash_map::IntoIter<HeaderName, Vec<HeaderValue>>,
}

impl Iterator for IntoIter {
    type Item = (HeaderName, Vec<HeaderValue>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
