#![feature(fn_traits)]

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

mod tests;

#[derive(Debug, Default, Clone)]
pub struct ErgoMap<T, S = RandomState> {
    map: HashMap<Id<T>, T, S>,
    inc: usize,
}

impl<T> ErgoMap<T, RandomState> {
    /// Creates an empty `ErgoMap`.
    pub fn new() -> Self {
        ErgoMap {
            map: HashMap::new(),
            inc: 0,
        }
    }

    /// Creates an empty `ErgoMap` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        ErgoMap {
            map: HashMap::with_capacity(capacity),
            inc: 0,
        }
    }
}

impl<T, S: BuildHasher> ErgoMap<T, S> {
    /// Creates an empty `ErgoMap` which will use the given hash builder to hash keys.
    ///
    /// Warning: `hash_builder` is normally randomly generated, and is designed to allow
    /// [`HashMap`]s to be resistant to attacks that cause many collisions and very poor
    /// performance. Setting it manually using this function can expose a DoS attack vector.
    pub fn with_hasher(hash_builder: S) -> Self {
        ErgoMap {
            map: HashMap::with_hasher(hash_builder),
            inc: 0,
        }
    }

    /// Creates an empty `ErgoMap`  with the specified capacity, using `hash_builder` to hash the
    /// keys.
    ///
    /// Warning: `hash_builder` is normally randomly generated, and is designed to allow
    /// [`HashMap`]s to be resistant to attacks that cause many collisions and very poor
    /// performance. Setting it manually using this function can expose a DoS attack vector.
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        ErgoMap {
            map: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            inc: 0,
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.map.clear()
    }

    /// Inserts a value into the map and returns the [`Id`] that can be used to access it.
    pub fn insert(&mut self, value: T) -> Id<T> {
        let id = Id::new_for(self);

        self.map.insert(id.clone(), value);
        id
    }

    /// Removes a value from the map by corresponding [`Id`], returning it if was found in the map.
    pub fn remove(&mut self, id: &Id<T>) -> Option<T> {
        self.map.remove(id)
    }

    /// Returns `true` if the map contains a value with the specified [`Id`].
    pub fn contains_id(&self, id: &Id<T>) -> bool {
        self.map.contains_key(id)
    }

    /// Returns a reference to the value corresponding to the [`Id`].
    pub fn get(&self, id: &Id<T>) -> Option<&T> {
        self.map.get(id)
    }

    /// Calls the given function on the corresponding value to the specified [`Id`].
    pub fn for_one<R, F: FnOnce(&T) -> R>(&self, id: &Id<T>, f: F) -> Option<R> {
        self.map.get(id).map(f)
    }

    /// Calls the given function on the corresponding value to the specified [`Id`]. Provides a
    /// mutable reference to the value.
    pub fn for_one_mut<R, F: FnOnce(&mut T) -> R>(&mut self, id: &Id<T>, f: F) -> Option<R> {
        self.map.get_mut(id).map(f)
    }

    /// Calls the given function on every Id-value pair in the map.
    pub fn for_all<F: FnMut(&Id<T>, &T)>(&self, mut f: F) {
        for args in self.map.iter() {
            f.call_mut(args)
        }
    }

    /// Calls the given function on every id-value pair in the map. Provides a mutable reference to
    /// values.
    pub fn for_all_mut<F: FnMut(&Id<T>, &mut T)>(&mut self, mut f: F) {
        for args in self.map.iter_mut() {
            f.call_mut(args)
        }
    }

    /// Chainable variant of `insert`.
    ///
    /// Not sure how useful this is because it doesn't return the [`Id`].
    #[cfg(feature = "chainable")]
    pub fn with(mut self, value: T) -> Self {
        self.insert(value);
        self
    }

    /// Chainable variant of `remove`.
    #[cfg(feature = "chainable")]
    pub fn without(mut self, id: &Id<T>) -> Self {
        self.remove(id);
        self
    }

    /// Chainable variant of `clear`.
    #[cfg(feature = "chainable")]
    pub fn cleared(mut self) -> Self {
        self.map.clear();
        self
    }
}

#[derive(Debug)]
pub struct Id<T> {
    key: Vec<u8>,
    phantom: PhantomData<T>,
}

// Not sure why these can't be derived but it won't compile unless I manually implement them.
// Seems to be a known issue. See https://github.com/rust-lang/rust/issues/26925.
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id::new(self.key.clone())
    }
}

impl<T> Id<T> {
    fn new(value: Vec<u8>) -> Self {
        Id {
            key: value,
            phantom: Default::default(),
        }
    }

    fn new_for<S>(map: &mut ErgoMap<T, S>) -> Self {
        map.inc += 1;
        Id {
            key: map.inc.to_be_bytes().to_vec(),
            phantom: Default::default(),
        }
    }
}
