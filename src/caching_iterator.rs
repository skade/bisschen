use std::iter::*;
use std::vec::*;

struct LazyCache<I, T> {
  priv iter: I,
  items: ~[T]
}

impl<I: Iterator<T>, T: Clone> LazyCache<I, T> {
  fn new(iter: I) -> LazyCache<I, T> {
    let items: ~[T] = ~[];
    LazyCache { iter: iter, items: items }
  }

  fn ensure(&mut self, end: uint) -> uint {
    let current_max_idx = self.items.len();
    if current_max_idx < end {
      let mut items_to_take = end - current_max_idx;
      while items_to_take > 0 {
        let next = self.iter.next();
        match next {
          Some(x) => { self.items.push(x); items_to_take -= 1 }
          None => { items_to_take = 0 }
        }
      }
    }
    self.items.len()
  }

  fn available(&self, idx: uint) -> bool {
    self.items.len() >= idx
  }

  fn idx<'a>(&'a self, idx: uint) -> Option<&'a T> {
    self.items.iter().idx(idx)
  }

  fn slice<'a>(&'a self, start: uint, end: uint) -> &'a [T] {
    if self.available(end) {
      self.items.slice(start, end)
    } else {
      fail!("accessed unloaded slice!")
    }
  }
}

#[test]
fn test_next_adds_to_items() {
  let items = ~[1, 2, 3];
  let mut cache = LazyCache::new(items.iter());
  cache.ensure(3);

  let slice = cache.slice(0,1);
  assert_eq!(1, slice.len());

  let slice2 = cache.slice(0,3);
  assert_eq!(3, slice2.len());
}

#[test]
fn test_returns_highest_index() {
  let items = &[1, 2, 3];
  let mut cache = LazyCache::new(items.iter());
  let highest_index = cache.ensure(300);

  let slice = cache.slice(0,highest_index);
  assert_eq!(slice.len(), cache.items.len())
}