//! Small insertion-ordered map and set.
//!
//! Importer maps hold a handful of entries. One linear implementation keeps
//! the WASM plugin far smaller than instantiating the standard hash and tree
//! maps for every key and value type.

use std::borrow::Borrow;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct Map<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Map {
            entries: Vec::new(),
        }
    }
}

impl<K: PartialEq, V> Map<K, V> {
    pub fn new() -> Self {
        Map::default()
    }

    fn position<Q: PartialEq + ?Sized>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
    {
        self.entries
            .iter()
            .position(|(existing, _)| existing.borrow() == key)
    }

    pub fn get<Q: PartialEq + ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.position(key).map(|index| &self.entries[index].1)
    }

    pub fn get_mut<Q: PartialEq + ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
    {
        self.position(key).map(|index| &mut self.entries[index].1)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.position(&key) {
            Some(index) => Some(std::mem::replace(&mut self.entries[index].1, value)),
            None => {
                self.entries.push((key, value));
                None
            }
        }
    }

    pub fn remove<Q: PartialEq + ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
    {
        self.position(key).map(|index| self.entries.remove(index).1)
    }

    pub fn or_insert_with(&mut self, key: K, make: impl FnOnce() -> V) -> &mut V {
        let index = match self.position(&key) {
            Some(index) => index,
            None => {
                self.entries.push((key, make()));
                self.entries.len() - 1
            }
        };
        &mut self.entries[index].1
    }

    pub fn or_default(&mut self, key: K) -> &mut V
    where
        V: Default,
    {
        self.or_insert_with(key, V::default)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.entries.iter().map(|(key, _)| key)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.iter().map(|(_, value)| value)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries.iter().map(|(key, value)| (key, value))
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl<K: PartialEq, V> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(items: I) -> Self {
        let mut map = Map::new();
        for (key, value) in items {
            map.insert(key, value);
        }
        map
    }
}

impl<K: PartialEq + Borrow<Q>, Q: PartialEq + ?Sized, V> Index<&Q> for Map<K, V> {
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("key is present")
    }
}

#[derive(Clone, Debug)]
pub struct Set<K> {
    items: Vec<K>,
}

impl<K> Default for Set<K> {
    fn default() -> Self {
        Set { items: Vec::new() }
    }
}

impl<K: PartialEq> Set<K> {
    pub fn new() -> Self {
        Set::default()
    }

    /// Add an item; true when it was not present.
    pub fn insert(&mut self, item: K) -> bool {
        if self.items.contains(&item) {
            return false;
        }
        self.items.push(item);
        true
    }

    pub fn contains(&self, item: &K) -> bool {
        self.items.contains(item)
    }

    pub fn is_disjoint(&self, other: &Set<K>) -> bool {
        !self.items.iter().any(|item| other.contains(item))
    }

    pub fn extend_from(&mut self, other: &Set<K>)
    where
        K: Clone,
    {
        for item in &other.items {
            self.insert(item.clone());
        }
    }
}
