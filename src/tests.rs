#![cfg(test)]

use crate::ErgoMap;

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
    let mut map = ErgoMap::with_capacity(64);
    map.insert(1);
    map.insert(1);
    map.insert(1);
    map.insert(1);
    assert_eq!(map.len(), 4)
}

#[test]
fn test_ergomap_clear() {
    let mut map = ErgoMap::with_capacity(64);
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
    let mut map = ErgoMap::with_capacity(64);
    assert!(map.is_empty());
    map.insert(1);
    assert!(!map.is_empty());
}

