#![allow(dead_code, unused)]

#[macro_use]
pub mod array {
    use std::ops::{Index, IndexMut};
    use std::vec::{IntoIter};
    use std::cmp::{max, min, Ordering};
    
    #[derive(Debug, Clone)]
    pub struct Array<T> {
        vec: Vec<T>,
    }

    pub struct ArrayEntry<'a, T>(usize, &'a T);

    impl<T> Array<T> {
        pub const fn new() -> Self {
            return Array { vec: vec![] };
        }

        pub const fn is_array(_: &Array<T>) -> bool {
            return true;
        }
    }

    impl<T> Array<T> {
        pub fn at(&self, index: isize) -> Option<&T> {
            if index < 0 {
                if index.abs() < self.vec.len().try_into().unwrap() {
                    return None;
                }

                return Some(&self.vec[self.vec.len() - index.abs() as usize]);
            }

            if index > self.vec.len().try_into().unwrap() {
                return None;
            }

            return Some(&self.vec[index as usize]);
        }

        pub fn length(&self) -> usize {
            return self.vec.len();
        }

        pub fn entries(&self) -> IntoIter<ArrayEntry<T>> {
            let mut v = vec![];

            for i in 0..self.vec.len() {
                v.push(ArrayEntry(i, &self.vec[i]));
            }

            return v.into_iter();
        }

        pub fn every<F>(&self, mut predicate: F) -> bool where F: FnMut(&T) -> bool {
            for i in 0..self.vec.len() {
                if !predicate(&self.vec[i]) {
                    return false;
                }
            }

            return true;
        }

        pub fn find<F>(&self, mut predicate: F) -> Option<&T> where F: FnMut(&T) -> bool {
            for i in 0..self.vec.len() {
                if predicate(&self.vec[i]) {
                    return Some(&self.vec[i]);
                }
            }

            return None;
        }

        pub fn find_index<F>(&self, mut predicate: F) -> Option<usize> where F: FnMut(&T) -> bool {
            for i in 0..self.vec.len() {
                if predicate(&self.vec[i]) {
                    return Some(i);
                }
            }

            return None;
        }

        pub fn for_each<F>(&self, mut predicate: F) -> () where F: FnMut(&T) -> () {
            for i in 0..self.vec.len() {
                predicate(&self.vec[i]);
            }

            return ();
        }

        pub fn keys(&self) -> IntoIter<usize> {
            let mut v = vec![];

            for i in 0..self.vec.len() {
                v.push(i);
            }

            return v.into_iter();
        }

        pub fn pop(&mut self) -> Option<T> {
            return self.vec.pop();
        }

        pub fn push(&mut self, v: T) -> usize {
            self.vec.push(v);

            return self.vec.len();
        }

        pub fn reduce() {
            unimplemented!("Reduce is too hard to implement :(");
        }

        pub fn reduce_right() {
            unimplemented!("Reduce right is too hard to implement :(");
        }

        pub fn reverse(mut self) -> Self {
            self.vec = self.vec.into_iter().rev().collect();

            return self;
        }

        pub fn shift(&mut self) -> Option<T> {
            if self.vec.len() == 0 {
                return None;
            }

            return Some(self.vec.remove(0));
        }

        pub fn some<F>(&self, mut predicate: F) -> bool where F: FnMut(&T) -> bool {
            for i in 0..self.vec.len() {
                if predicate(&self.vec[i]) {
                    return true;
                }
            }

            return false;
        }

        pub fn splice() {
            unimplemented!("Splice is too hard to implement :(");
        }

        pub fn unshift(&mut self, v: T) -> usize {
            self.vec.insert(0, v);

            return self.vec.len();
        }
    }

    impl<T> Array<T> where T: Clone {
        pub fn from_vec(vec: &Vec<T>) -> Self {
            return Array { vec: vec.to_vec() };
        }

        pub fn from_fixed_array(array: &[T]) -> Self {
            return Array { vec: array.to_vec() };
        }
    }

    impl<T> Array<T> where T: Clone {
        pub fn concat(&self, a: &Array<T>) -> Self {
            Array { vec: self.vec.iter().cloned().chain(a.vec.iter().cloned()).collect() }
        }

        pub fn copy_within<E>(mut self, target: isize, start: isize, end: E) -> Self where E: Into<Option<isize>> {
            let end = end.into().unwrap_or(self.length().try_into().unwrap());

            let len = self.length() as isize;

            let mut to =
            if target < 0 {
                max(len - target.abs(), 0)
            } else {
                min(target, len)
            };

            let mut from =
            if start < 0 {
                max(len - start.abs(), 0)
            } else {
                min(start, len)
            };

            let fnl =
            if end < 0 {
                max(len - end.abs(), 0)
            } else {
                min(end, len)
            };

            let mut count = min(fnl - from, len - to);

            let direction = if from < to && to < from + count {
                from += count - 1;
                to += count - 1;
                -1
            } else { 1 };

            while count > 0 {
                let v = self.vec[from as usize].clone();

                self.vec[to as usize] = v;

                from += direction;
                to += direction;
                count -= 1;
            }

            return self;
        }

        pub fn fill<S, E>(mut self, value: &T, start: S, end: E) -> Self where S: Into<Option<isize>>, E: Into<Option<isize>> {
            let start = start.into().unwrap_or(0 as isize);
            let end = end.into().unwrap_or(self.length().try_into().unwrap());

            let len = self.length() as isize;

            let mut k =
            if start < 0 {
                max(len + start, 0)
            } else {
                min(start, len)
            };

            let fnl =
            if end < 0 {
                max(len + end, 0)
            } else {
                min(end, len)
            };

            while k < fnl {
                self.vec[k as usize] = value.clone();

                k = k + 1
            }

            return self;
        }

        pub fn filter<F>(&self, mut predicate: F) -> Self where F: FnMut(&T) -> bool {
            let mut v = vec![];

            for i in 0..self.vec.len() {
                if predicate(&self.vec[i]) {
                    v.push(self.vec[i].clone());
                }
            }

            return Array::from_vec(&v);
        }

        pub fn flat_map<R, F>(&self, mut predicate: F) -> Array<R> where R: Clone, F: FnMut(&T) -> Array<R> {
            let mut v = vec![];

            for i in 0..self.vec.len() {
                let a = predicate(&self.vec[i]);

                for j in 0..a.length() {
                    v.push(a.vec[j].clone());
                }
            }

            return Array::from_vec(&v);
        }

        pub fn map<R, F>(&self, mut predicate: F) -> Array<R> where R: Clone, F: FnMut(&T) -> R {
            let mut v = vec![];

            for i in 0..self.vec.len() {
                v.push(predicate(&self.vec[i]));
            }

            return Array::from_vec(&v);
        }

        pub fn slice<S, E>(&self, start: S, end: E) -> Array<T> where S: Into<Option<isize>>, E: Into<Option<isize>> {
            let start = start.into().unwrap_or(0);
            let end = end.into().unwrap_or(self.length().try_into().unwrap());

            let len = self.length() as isize;

            let mut k =
            if start < 0 {
                max(len + start, 0)
            } else {
                min(start, len)
            };

            let fnl =
            if end < 0 {
                max(len + end, 0)
            } else {
                min(end, len)
            };

            let count = max(fnl - k, 0);

            let mut v = vec![];

            while k < fnl {
                v.push(self.vec[k as usize].clone());

                k += 1;
            }

            return Array::from_vec(&v);
        }

        pub fn values(&self) -> IntoIter<T> {
            let mut v = vec![];

            for i in 0..self.vec.len() {
                v.push(self.vec[i].clone());
            }

            return v.into_iter();
        }
    }

    impl<T> Array<T> where T: PartialEq {
        pub fn includes<I>(&self, v: T, index: I) -> bool where I: Into<Option<isize>> {
            let index = index.into().unwrap_or(0);

            if index.abs() as usize > self.length() {
                return false;
            }

            let index =
            if index < 0 {
                self.length() - index.abs() as usize
            } else {
                index as usize
            };

            for i in index..self.vec.len() {
                if self.vec[i] == v {
                    return true;
                }
            }

            return false;
        }

        pub fn index_of<I>(&self, v: T, index: I) -> Option<usize> where I: Into<Option<isize>> {
            let index = index.into().unwrap_or(0);

            if index.abs() as usize > self.length() {
                return None;
            }

            let index =
            if index < 0 {
                self.length() - index.abs() as usize
            } else {
                index as usize
            };

            for i in index..self.vec.len() {
                if self.vec[i] == v {
                    return Some(i);
                }
            }

            return None;
        }

        pub fn last_index_of<I>(&self, v: T, index: I) -> Option<usize> where I: Into<Option<isize>> {
            let index = index.into().unwrap_or(self.length() as isize);

            if index.abs() as usize > self.length() {
                return None;
            }
                
            let index =
            if index < 0 {
                self.length() - index.abs() as usize
            } else {
                index as usize
            };

            for i in self.vec.len() - 1..=index {
                if self.vec[i] == v {
                    return Some(i);
                }
            }

            return None;
        }
    }

    impl<T> Array<T> where T: Eq + Ord + PartialEq + PartialOrd {
        pub fn sort<F>(mut self, cmp: F) -> Self where F: FnMut(&T, &T) -> Ordering {
            self.vec.sort_by(cmp);

            return self;
        }
    }

    impl<T> Array<T> where T: ToString {
        pub fn join(&self, seperator: String) -> String {
            return self.vec.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(&seperator);
        }
    }

    impl<T> Array<Array<T>> where T: Clone {
        pub fn flat(&self) -> Array<T> {
            let mut v = vec![];

            for i in 0..self.vec.len() {
                for j in 0..self.vec[i].length() {
                    v.push(self[i][j].clone());
                }
            }

            return Array::from_vec(&v);
        }
    }

    impl<T> Default for Array<T> {
        fn default() -> Self {
            return Array::new();
        }
    }

    impl<T> ToString for Array<T> where T: ToString {
        fn to_string(&self) -> String {
            return format!("[{}]", self.vec.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", "));
        }
    }

    impl<T> Index<usize> for Array<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            return &self.vec[index];
        }
    }

    impl<T> IndexMut<usize> for Array<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            return &mut self.vec[index];
        }
    }

    macro_rules! array {
        () => (
            $crate::array::Array::new()
        );
        ($elem:expr; $n:expr) => (
            $crate::array::Array::from_vec(vec![$elem; $n])
        );
        ($($x:expr),+ $(,)?) => ({
            let mut vec = vec![];
            $( vec.push($x); )*
            $crate::array::Array::from_vec(vec)
        });
    }
}

fn main() {
    use array::*;

    // 
}
