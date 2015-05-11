//! Provides an adapter to read sequentially from the items of an
//! iterator.

use std::io::{Read, Result};

/// Creates an instance of a reader that reads sequentially from the
/// items of an iterator.
pub fn cat<R, I>(mut iter: I) -> Cat<R, I>
where R: Read, I: Iterator<Item=R> {
    let curr = iter.next();

    Cat {
        iter: iter,
        curr: curr,
    }
}

/// A reader that reads sequentially from the items in an iterator.
pub struct Cat<R, I>
where R: Read, I: Iterator<Item=R> {
    iter: I,
    curr: Option<R>,
}

impl<R, I> Cat<R, I>
where R: Read, I: Iterator<Item=R> {
    /// Returns a reference to the item last read, or None if the
    /// iterator has been exhausted.
    ///
    /// This is useful for error handling and reporting: if a read
    /// operation fails, the reference returned will point to the item
    /// which caused the the error.
    pub fn current(&self) -> Option<&R> {
        self.curr.as_ref()
    }
}

impl<R, I> Read for Cat<R, I>
where R: Read, I: Iterator<Item=R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = match self.curr {
            None => 0,
            Some(ref mut r) => try!(r.read(buf)),
        };

        if n > 0 || buf.len() == 0 || self.curr.is_none() {
            Ok(n)
        } else {
            // The current reader reached the end so we have to advance
            // the iterator and try again.
            self.curr = self.iter.next();
            self.read(buf)
        }
    }
}
