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

use super::{
    BinaryHeap,
};
use crate::storage2::traits::PackedLayout;
use core::{
    cmp::{PartialEq, Eq},
    iter::{
        Extend,
        FromIterator,
    }
};

impl<T> Default for BinaryHeap<T>
where
    T: PackedLayout,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Extend<T> for BinaryHeap<T>
where
    T: PackedLayout,
{
    fn extend<I>(&mut self, iter: I)
        where
            I: IntoIterator<Item = T>,
    {
        for item in iter {
            self.push(item)
        }
    }
}

impl<T> FromIterator<T> for BinaryHeap<T>
    where
        T: PackedLayout,
{
    fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = T>,
    {
        let mut vec = Self::new();
        vec.extend(iter);
        vec
    }
}

impl<T> PartialEq for BinaryHeap<T>
where
    T: PartialEq + PackedLayout,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false
        }
        self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs == rhs)
    }
}

impl<T> Eq for BinaryHeap<T> where T: Eq + PackedLayout {}
