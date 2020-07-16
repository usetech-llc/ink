// Copyright 2019-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Binary heap data structure and utilities.
//!
//! todo: more module description (see original bit_stash and other collections)

mod impls;

#[cfg(test)]
mod tests;

use crate::storage2::traits::PackedLayout;
use super::vec::{
    Vec as StorageVec,
    Iter,
    IterMut,
};

/// A binary heap type, providing `O(log(n))` push and pop operations.
///
/// # Note
///
/// The heap is a *max-heap* by default, i.e. the first element is the largest.
/// In order to make it a *min-heap*, implement the `Ord` trait explicitly on the type
/// which is stored in the heap.
#[derive(Debug)]
pub struct BinaryHeap<T>
where
    T: PackedLayout + Ord,
{
    /// The synchronized cells to operate on the contract storage.
    elems: StorageVec<T>,
}

impl<T> BinaryHeap<T>
where
    T: PackedLayout + Ord,
{
    /// Creates a new empty storage heap.
    pub fn new() -> Self {
        Self {
            elems: StorageVec::new(),
        }
    }

    /// Returns the number of elements in the heap, also referred to as its 'length'.
    pub fn len(&self) -> u32 {
        self.elems.len()
    }

    /// Returns `true` if the heap contains no elements.
    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }
}

impl<T> BinaryHeap<T>
where
    T: PackedLayout + Ord,
{
    /// Returns an iterator yielding shared references to all elements of the heap.
    ///
    /// # Note
    ///
    /// Avoid unbounded iteration over large heaps.
    /// Prefer using methods like `Iterator::take` in order to limit the number
    /// of yielded elements.
    pub fn iter(&self) -> Iter<T> {
        self.elems.iter()
    }

    /// Returns an iterator yielding exclusive references to all elements of the heap.
    ///
    /// # Note
    ///
    /// Avoid unbounded iteration over big storage vectors.
    /// Prefer using methods like `Iterator::take` in order to limit the number
    /// of yielded elements.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.elems.iter_mut()
    }

    /// Returns a shared reference to the greatest element of the heap
    ///
    /// Returns `None` if the heap is empty
    pub fn peek(&self) -> Option<&T> {
        self.elems.first()
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    fn sift_down(&mut self, mut pos: u32) {
        let end = self.len();
        let mut child = 2 * pos + 1;
        while child < end {
            let right = child + 1;
            // compare with the greater of the two children
            if right < end && self.elems.get(child) <= self.elems.get(right) {
                child = right;
            }
            // if we are already in order, stop.
            if self.elems.get(pos) >= self.elems.get(child) {
                break;
            }
            pos = child;
            child = 2 * pos + 1;
        }
    }

    /// Pops greatest element from the heap and returns it
    ///
    /// Returns `None` if the heap is empty
    pub fn pop(&mut self) -> Option<T> {
        // replace the root of the heap with the last element
        let elem = self.elems.swap_remove(0);
        self.sift_down(0);
        elem
    }
}

impl<T> BinaryHeap<T>
where
    T: PackedLayout + Ord,
{
    // todo: optimize!
    fn sift_up(&mut self, index: u32) {
        assert!(
            index > 0,
            "cannot bubble up the root element"
        );
        let parent_index = (index - 1) / 2;
        let parent = self.elems.get(parent_index)
            .expect("parent must exist in fully compacted sequence of elements");
        let child = self.elems.get(index)
            .expect("child must exist, either just inserted or a previous parent");

        if child > parent {
            self.elems.swap(parent_index, index);
            self.sift_up(parent_index);
        }
    }

    /// Pushes the given element to the binary heap.
    pub fn push(&mut self, value: T) {
        let old_len = self.len();
        self.elems.push(value);
        self.sift_up(old_len)
    }
}
