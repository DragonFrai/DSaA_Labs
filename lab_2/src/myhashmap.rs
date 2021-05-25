use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

const INIT_SIZE: usize = 16;

pub struct ReHashMap<T> {
    inner: Vec<Option<T>>,
}

impl<T> ReHashMap<T> {
    pub fn new(size: usize) -> Self { Self { inner: (0..size).map(|_| None).collect() } }

    pub fn add(&mut self, value: T) where T: Hash + Eq {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();

        for i in 0..self.inner.len() {
            let size = self.inner.len();
            let idx = ((hash as usize) % size + i) % size;
            if self.inner[idx].is_none() {
                self.inner[idx] = Some(value);
                return;
            }
            if self.inner[idx].is_some() && *self.inner[idx].as_ref().unwrap() == value {
                return;
            }
        }
        panic!("Хеш-карта заполнена.")
    }

    pub fn has_value(&self, value: &T) -> bool where T: Hash + Eq {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();

        for i in 0..self.inner.len() {
            let size = self.inner.len();
            let idx = ((hash as usize) % size + i) % size;
            match &self.inner[idx] {
                Some(x) => if x.eq(value) { return true },
                _ => (),
            }
        }
        false
    }
}


struct Chain<T> {
    value: T,
    next: Option<Box<Chain<T>>>,
}

impl<T> Chain<T> {
    pub fn find(&self, value: &T) -> bool where T: Eq {
        if self.value == *value {
            true
        } else if let Some(next) = &self.next {
            next.find(value)
        } else {
            false
        }
    }

    pub fn put(&mut self, value: T) {
        match &mut self.next {
            Some(next) => next.put(value),
            None => self.next = Some(Box::new(Chain { value, next: None })),
        }
    }
}

pub struct ChainHashMap<T> {
    inner: Vec<Option<Chain<T>>>,
}

impl<T> ChainHashMap<T> {
    pub fn new(size: usize) -> Self { Self { inner: (0..size).map(|_| None).collect() } }

    pub fn add(&mut self, value: T) where T: Hash + Eq {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        let idx = (hash as usize) % self.inner.len();

        match &mut self.inner[idx] {
            Some(chain) => chain.put(value),
            None => self.inner[idx] = Some(Chain { value, next: None }),
        }
    }

    pub fn has_value(&self, value: &T) -> bool where T: Hash + Eq {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        let idx = (hash as usize) % self.inner.len();

        match &self.inner[idx] {
            Some(chain) => chain.find(value),
            None => false,
        }
    }
}

