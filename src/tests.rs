#![cfg(test)]

use crate::{ErgoMap, Id};
use std::any::{Any, TypeId};

#[test]
fn test_ergomap_contains_id() {
    let mut map = ErgoMap::new();
    let id = map.insert(1);
    assert!(map.contains_id(&id))
}

#[test]
fn test_ergomap_remove() {
    let mut map = ErgoMap::new();
    let id = map.insert(1);
    map.remove(&id);
    assert!(!map.contains_id(&id))
}

#[test]
fn test_ergomap_capacity() {
    let mut map = ErgoMap::with_capacity(64);
    map.insert(1);
    assert!(map.capacity() >= 64)
}

#[test]
fn test_ergomap_len() {
    let mut map = ErgoMap::new();
    map.insert(1);
    map.insert(1);
    map.insert(1);
    map.insert(1);
    assert_eq!(map.len(), 4)
}

#[test]
fn test_ergomap_clear() {
    let mut map = ErgoMap::new();
    map.insert(1);
    map.insert(1);
    map.insert(1);
    map.insert(1);
    map.clear();
    assert_eq!(map.len(), 0);
    assert!(map.is_empty())
}

#[test]
fn test_ergomap_is_empty() {
    let mut map = ErgoMap::new();
    assert!(map.is_empty());
    map.insert(1);
    assert!(!map.is_empty());
}

#[test]
fn test_ergomap_get() {
    let mut map = ErgoMap::new();
    let id1 = map.insert(1);
    let id2 = map.insert(2);
    let id3 = map.insert(3);
    assert_eq!(map.get(&id1).unwrap().clone(), 1);
    assert_eq!(map.get(&id2).unwrap().clone(), 2);
    assert_eq!(map.get(&id3).unwrap().clone(), 3);
}

#[test]
fn test_ergomap_for_one() {
    let mut map: ErgoMap<i32> = ErgoMap::new();
    let id1 = map.insert(1);
    let id2 = map.insert(2);
    let id3 = map.insert(3);

    assert_eq!(map.for_one(&id1, |value| { value.pow(0) }).unwrap(), 1);
    assert_eq!(map.for_one(&id2, |value| { value.pow(2) }).unwrap(), 4);
    assert_eq!(map.for_one(&id3, |value| { value.pow(4) }).unwrap(), 81);
}

#[test]
fn test_ergomap_for_one_mut() {
    struct IWrapper(i32);

    let mut map = ErgoMap::new();
    let id1 = map.insert(IWrapper(1));
    let id2 = map.insert(IWrapper(2));
    let id3 = map.insert(IWrapper(3));

    map.for_one_mut(&id1, |value| value.0 = 1);
    map.for_one_mut(&id2, |value| value.0 = 22);
    map.for_one_mut(&id3, |value| value.0 = 333);

    assert_eq!(map.get(&id1).unwrap().0, 1);
    assert_eq!(map.get(&id2).unwrap().0, 22);
    assert_eq!(map.get(&id3).unwrap().0, 333);
}

#[test]
fn test_ergomap_for_all() {
    let mut map = ErgoMap::new();
    map.insert(1);
    map.insert(1);
    map.insert(1);

    map.for_all(|id, value| {
        assert_eq!(value.clone(), 1);
        assert_eq!(id.type_id(), TypeId::of::<Id<i32>>())
    })
}

#[test]
fn test_ergomap_for_all_mut() {
    struct IWrapper(i32);

    let mut map = ErgoMap::new();
    let id1 = map.insert(IWrapper(1));
    let id2 = map.insert(IWrapper(2));
    let id3 = map.insert(IWrapper(3));

    map.for_all_mut(|_, value| {
        value.0 = value.0.pow(2);
    });

    assert_eq!(map.get(&id1).unwrap().0, 1);
    assert_eq!(map.get(&id2).unwrap().0, 4);
    assert_eq!(map.get(&id3).unwrap().0, 9);
}
