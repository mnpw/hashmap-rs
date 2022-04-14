use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Index;

const INITIAL_NBUCKETS: usize = 1;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        // this should be 'free' to create
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

pub struct OccupiedEntry<'a, K, V> {
    element: &'a mut (K, V),
}
pub struct VacantEntry<'a, K, V> {
    key: K,
    bucket: &'a mut Vec<(K, V)>,
}
impl<'a, K, V> VacantEntry<'a, K, V> {
    fn insert(self, value: V) -> &'a mut V {
        self.bucket.push((self.key, value));
        &mut self.bucket.last_mut().unwrap().1
    }
}

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V> {
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            Entry::Occupied(e) => &mut e.element.1,
            Entry::Vacant(e) => e.insert(value),
        }
    }

    pub fn or_insert_with<F>(self, maker: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(e) => &mut e.element.1,
            Entry::Vacant(e) => e.insert(maker()),
        }
    }

    pub fn or_default<F>(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert(V::default())
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn bucket<Q>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash % self.buckets.len() as u64) as usize
    }

    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }
        let is_present = self.get(&key);

        if is_present.is_some() {
            self.entry_occupied(key)
        } else {
            self.entry_vacant(key)
        }
    }

    fn entry_occupied(&mut self, key: K) -> Entry<K, V> {
        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];
        let e = bucket
            .iter_mut()
            .find(|&&mut (ref k, _)| k == &key)
            .unwrap();

        Entry::Occupied(OccupiedEntry { element: e })
    }

    fn entry_vacant(&mut self, key: K) -> Entry<K, V> {
        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];
        Entry::Vacant(VacantEntry {
            key,
            bucket: bucket,
        })
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // if buckets is empty or it is 3/4th full the resize
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];
        self.items += 1;

        for (ref k, ref mut v) in bucket.iter_mut() {
            if k == &key {
                // update
                return Some(mem::replace(v, value));
            }
        }

        bucket.push((key, value));
        None
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|&(k, _)| k.borrow() == key)
            .map(|&(_, ref v)| v)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket(&key);
        self.buckets[bucket]
            .iter()
            .find(|&(k, _)| k.borrow() == key)
            .is_some()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];

        let id = bucket.iter().position(|&(ref k, _)| k.borrow() == key)?;

        self.items -= 1;
        let (_, val) = bucket.swap_remove(id);
        Some(val)
    }

    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };

        // create new buckets
        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        // add elements into new buckets
        for (key, val) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let hash = hasher.finish();

            let bucket = (hash % new_buckets.len() as u64) as usize;
            let bucket = &mut new_buckets[bucket];
            bucket.push((key, val));
        }

        // replace old buckets with new buckets
        let _ = mem::replace(&mut self.buckets, new_buckets);
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn from(entries: [(K, V); N]) -> Self {
        Self::from_iter(entries)
    }
}

impl<Q, K, V> Index<&Q> for HashMap<K, V>
where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq + ?Sized,
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).expect("no entry found for the key")
    }
}

pub struct Iter<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => match bucket.get(self.at) {
                    Some(&(ref key, ref val)) => {
                        self.at += 1;
                        break Some((key, val));
                    }
                    None => {
                        self.bucket += 1;
                        self.at = 0;
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            map: self,
            bucket: 0,
            at: 0,
        }
    }
}

impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = HashMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        map.insert("foo", 42);
        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.remove(&"foo"), Some(42));
        assert_eq!(map.get(&"foo"), None);
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn iter() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 10);
        map.insert("c", 100);
        map.insert("d", 1000);

        for (&k, &v) in &map {
            match k {
                "a" => assert_eq!(v, 1),
                "b" => assert_eq!(v, 10),
                "c" => assert_eq!(v, 100),
                "d" => assert_eq!(v, 1000),
                _ => unreachable!(),
            }
        }

        assert_eq!((&map).into_iter().count(), 4);
    }

    #[test]
    fn index() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert_eq!(map["foo"], 42);

        let result = std::panic::catch_unwind(|| map["bar"]);
        assert!(result.is_err());
    }

    #[test]
    fn entry() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        *map.entry("foo").or_insert(42) += 1;

        assert_eq!(map["foo"], 43);

        *map.entry("bar").or_insert(70) -= 1;
        assert_eq!(map["bar"], 69);
    }
}
