use super::*;
use std::collections::VecDeque;

pub struct CircularVec<T> {
    size: usize,
    inner: VecDeque<T>,
}

impl<T> CircularVec<T> {
    pub fn new(n: usize) -> CircularVec<T> {
        CircularVec {
            size: n,
            inner: VecDeque::with_capacity(n),
        }
    }

    pub fn push(&mut self, t: T) {
        while self.inner.len() >= self.size {
            self.inner.pop_front();
        }
        self.inner.push_back(t);
    }

    pub fn get(&self) -> &VecDeque<T> {
        &self.inner
    }
}
