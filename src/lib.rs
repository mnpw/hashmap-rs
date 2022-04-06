use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

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

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // if buckets is empty or it is 3/4th full the resize
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        let bucket = (hash % self.buckets.len() as u64) as usize;
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
        mem::replace(&mut self.buckets, new_buckets);
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
    }
}
