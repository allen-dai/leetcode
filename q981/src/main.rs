use std::collections::HashMap;

fn main() {}

struct TimeMap {
    obj: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            obj: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if let Some(val) = self.obj.get_mut(&key) {
            val.push((timestamp, value));
        } else {
            self.obj.insert(key, vec![(timestamp, value)]);
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(val) = self.obj.get(&key) {
            let (mut l, mut r) = (0, val.len());
            let mut res = "".to_string();
            while l < r {
                let m = (l + r) / 2;
                if val[m].0 < timestamp {
                    res = val[m].1.to_string();
                    l = m + 1;
                } else {
                    r = m;
                }

                return res;
            }
        }

        "".to_string()
    }
}
