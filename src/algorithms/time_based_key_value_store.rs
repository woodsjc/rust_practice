use std::collections::HashMap;

struct TimeMap {
    hm: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap { hm: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if let Some(v) = self.hm.get_mut(&key) {
            v.push((value, timestamp));
            //v.sort_by_key(|x| x.1);
        } else {
            self.hm.insert(key, vec![(value, timestamp)]);
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(v) = self.hm.get(&key) {
            let mut l = 0;
            let mut r = v.len() - 1;
            while l < r {
                let m = l + (r - l + 1) / 2;
                if v[m].1 <= timestamp {
                    l = m;
                } else {
                    r = m - 1;
                }
            }
            if v[l].1 <= timestamp {
                return v[l].1.to_string();
            }
        }
        "".to_string()
    }
}
