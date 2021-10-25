use std::collections::HashMap;

pub struct LruMap<T, K, const S: usize> {
    map: HashMap<K, (T, u32)>,
    counter: u32,
}

impl<T, K, const S: usize> LruMap<T, K, S>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            counter: 0,
        }
    }

    pub fn get(&mut self, key: K) -> Option<&T> {
        let val = self.map.get_mut(&key);
        if let Some((t, counter)) = val {
            self.counter += 1;
            *counter = self.counter;
            return Some(t);
        }
        None
    }

    pub fn put(&mut self, key: K, t: T) {
        if self.map.len() == S {
            let lru_key = self.get_lru_key();
            if let Some(lru_key) = lru_key {
                self.map.remove(&lru_key);
            }
        }
        self.counter += 1;
        self.map.insert(key, (t, self.counter));
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.counter = 0;
    }

    fn get_lru_key(&self) -> Option<K> {
        let mut lru_key: Option<K> = None;
        let mut lru_counter = u32::MAX;
        for (k, val) in self.map.iter() {
            if val.1 < lru_counter {
                lru_key = Some(k.clone());
                lru_counter = val.1;
            }
        }
        lru_key
    }
}

mod tests {

    #[test]
    fn test_lru() {
        use super::LruMap;
        let mut list: LruMap<u32, u32, 3> = LruMap::new();

        assert!(list.get(3).is_none());
        list.put(3, 6);
        assert_eq!(*list.get(3).unwrap(), 6);

        list.put(4, 8);
        list.put(5, 10);

        assert_eq!(*list.get(4).unwrap(), 8);
        assert_eq!(*list.get(5).unwrap(), 10);

        list.put(6, 12);

        assert_eq!(*list.get(6).unwrap(), 12);

        assert!(list.get(3).is_none());

        list.put(4, 4);
        list.put(7, 14);

        assert!(list.get(5).is_none());
        assert_eq!(*list.get(4).unwrap(), 4);

        list.clear();
        assert!(list.get(4).is_none());
    }
}
