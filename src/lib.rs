//! # ErgoMap
//!
//! **ErgoMap** is simple data structure library for wrapping the std [`HashMap`] in a way that
//! makes code easier to write and restricts key creation to reduce the amount of invalid `get`
//! calls. All [`ErgoMap`] objects use the [`Id`] type as a key. [`Id`] contains the correspond
//! value type as a generic parameter and has no public constructor. THis means that [`Id`] must be
//! obtained through [`ErgoMap`] method calls like `insert`.
//!
//! For more flexibility, values can be inserted into [`ErgoMap`] using a specified [`Vec<u8>`] as a
//! key. The provided [`Vec`] is converted into and returned as an [`Id`]. User types which
//! implement the [`BuildId`] trait can instead provided a [`Vec<u8>`] for [`Id`] creation
//! themselves.
//!
//! It also implements some methods not found on the std [`HashMap`] for functional programming
//! and chaining method calls.
//!
//! # License
//! ErgoMap is licensed under the [MIT license.](https://choosealicense.com/licenses/mit/) You can
//! access the source repository on [GitHub.](https://github.com/kalternate/ergomap)

#![feature(fn_traits)]
#![feature(split_array)]

use rand::{thread_rng, Rng};
use std::collections::hash_map::{Iter, IterMut, RandomState};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

mod tests;

/// Map that wraps the std [`HashMap`], using [`Id`] as the key.
///
/// When a value is inserted into the map, the corresponding [`Id`] is returned, which can be used
/// to access it latter. Thus, values can be inserted into [`ErgoMap`] without having to specify a
/// key. This restricts the pool of possible [`Id`]s. Note that invalid [`Id`]s can still exist,
/// either by removing values from the map or obtaining [`Id`]s from another map.
#[derive(Debug, Default, Clone)]
pub struct ErgoMap<T, S = RandomState> {
    map: HashMap<Id<T>, T, S>,
}

impl<T> ErgoMap<T, RandomState> {
    /// Creates an empty `ErgoMap`.
    pub fn new() -> Self {
        ErgoMap {
            map: HashMap::new(),
        }
    }

    /// Creates an empty `ErgoMap` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        ErgoMap {
            map: HashMap::with_capacity(capacity),
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

        self.map.insert(id, value);
        id
    }

    /// Inserts a value into the map, using the specified key [`Vec`] to make an [`Id`] for it.
    ///
    /// Returns [`None`] if that [`Id`] is already in use. Otherwise returns that [`Id`].
    pub fn insert_as(&mut self, key: Key, value: T) -> Option<Id<T>> {
        let id = Id::new(key);

        if self.contains_id(&id) {
            return None;
        }

        self.map.insert(id, value);
        Some(id)
    }

    /// Inserts a value into the map, using the specified key [`Vec`] to make an [`Id`] for it.
    ///
    /// If that [`Id`] is already in use, then the previous corresponding value is dropped.
    pub fn force_insert_as(&mut self, key: Key, value: T) -> Id<T> {
        let id = Id::new(key);
        self.map.insert(id, value);
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
    ///
    /// # Panics
    /// Panics if the [`Id`] is not used in the map.
    pub fn get(&self, id: &Id<T>) -> &T {
        self.map.get(id).unwrap()
    }

    /// Returns a mutable reference to the value corresponding to the [`Id`].
    ///
    /// # Panics
    /// Panics if the [`Id`] is not used in the map.
    pub fn get_mut(&mut self, id: &Id<T>) -> &mut T {
        self.map.get_mut(id).unwrap()
    }

    /// Returns a reference to the value corresponding to the [`Id`] or [`None`] if there is no
    /// corresponding value.
    pub fn try_get(&self, id: &Id<T>) -> Option<&T> {
        self.map.get(id)
    }

    /// Returns a mutable reference to the value corresponding to the [`Id`] or [`None`] if there
    /// is no corresponding value.
    pub fn try_get_mut(&mut self, id: &Id<T>) -> Option<&mut T> {
        self.map.get_mut(id)
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

impl<T: BuildId, S: BuildHasher> ErgoMap<T, S> {
    /// Inserts a value into the map, using it's `get_key` method to build an [`Id`] for it.
    ///
    /// Returns [`None`] if that [`Id`] is already in use. Otherwise returns that [`Id`].
    pub fn build_insert(&mut self, value: T) -> Option<Id<T>> {
        self.insert_as(value.get_key(), value)
    }

    /// Inserts a value into the map, using it's `get_key` method to build an [`Id`] for it.
    ///
    /// If that [`Id`] is already in use, then the previous corresponding value is dropped.
    pub fn force_build_insert(&mut self, value: T) -> Id<T> {
        self.force_insert_as(value.get_key(), value)
    }

    /// Returns an iterator visiting all id-value pairs in array order.
    pub fn iter(&self) -> Iter<'_, Id<T>, T> {
        self.map.iter()
    }

    /// Returns a mutable iterator visiting all id-value pairs in array order.
    pub fn iter_mut(&mut self) -> IterMut<'_, Id<T>, T> {
        self.map.iter_mut()
    }
}

/// Key used to access values in an [`ErgoMap`].
///
/// Constructors are made private to reduce the amount of invalid `get` calls. Note that `get` can
/// still return [`None`] if the value has been removed or the `Id` was made by a different map.
#[derive(Debug)]
pub struct Id<T> {
    key: RawKey,
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
        Id::new(Key::Array(self.key))
    }
}

impl<T> Copy for Id<T> {}

impl<T> Id<T> {
    fn new(key: Key) -> Self {
        Id {
            key: match key {
                Key::Random => thread_rng().gen(),
                Key::Value(value) => value.to_be_bytes(),
                Key::Array(slice) => slice,
                Key::Str(s) => {
                    let mut v: Vec<u8> = s.into();
                    while v.len() < 16 {
                        v.push(0x00)
                    }

                    *v.as_slice().split_array_ref().0
                }
            },
            phantom: Default::default(),
        }
    }

    fn new_for<S: BuildHasher>(map: &mut ErgoMap<T, S>) -> Self {
        let mut id = Id::new(Key::Random);

        while map.contains_id(&id) {
            id = Id::new(Key::Random);
        }

        id
    }
}

/// Types that implement this trait can make there own [`Id`] so that it will be constant across
/// executions and platforms.
pub trait BuildId {
    /// Returns a [`Key`] which will be used to make an [`Id`].
    ///
    /// The return value should be unique compared to other values entered into the [`ErgoMap`] but
    /// should be constant across executions and platforms.
    fn get_key(&self) -> Key;
}

type RawKey = [u8; 16];

/// Type used to make an [`Id`] from various types.
pub enum Key {
    /// Generates a random key using [`rand::rngs::ThreadRng`] to make the [`Id`].
    Random,
    /// Uses a 128-bit integer to make the [`Id`]
    Value(u128),
    /// Uses a 16-byte array to make the [`Id`].
    Array([u8; 16]),
    /// Uses a [`String`] to make the [`Id`].
    ///
    /// Note that at most only the first 16 bytes of the [`String`], encoded in UTF-8, will be used.
    Str(String),
}
