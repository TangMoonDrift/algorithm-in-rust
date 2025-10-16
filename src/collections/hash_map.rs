use std::collections::HashMap;
use std::time::Instant;

pub struct CustomHashMap<K, V> {
    map: HashMap<K, (V, u64)>,
    timestamp: u64,
    set_all_timestamp: u64,
    set_all_value: V,
}

impl<K, V> CustomHashMap<K, V>
where
    K: std::hash::Hash + std::cmp::Eq,
    V: std::default::Default + Copy,
{
    pub fn new() -> Self {
        let timestamp = Instant::now().elapsed().as_millis() as u64;
        CustomHashMap {
            map: HashMap::new(),
            timestamp,
            set_all_timestamp: 0,
            set_all_value: V::default(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        let timestamp = Instant::now().elapsed().as_millis() as u64;
        self.timestamp = timestamp;
        let record = (value, timestamp);
        if self.map.contains_key(&key) {
            return false;
        }
        self.map.insert(key, record);
        true
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.map.get(key) {
            Some(record) => {
                if record.1 > self.timestamp {
                    Some(&record.0)
                } else {
                    Some(&self.set_all_value)
                }
            }
            None => None,
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.map.remove(key) {
            Some(record) => {
                if record.1 > self.timestamp {
                    Some(record.0)
                } else {
                    Some(self.set_all_value)
                }
            }
            None => None,
        }
    }

    pub fn set_all(&mut self, value: V) {
        let timestamp = Instant::now().elapsed().as_millis() as u64;
        self.set_all_timestamp = timestamp;
        self.set_all_value = value;
    }
}
