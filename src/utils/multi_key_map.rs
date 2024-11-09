use std::collections::HashMap;

pub struct MultiKeyMap<K1, V> {
    values: Vec<V>,
    map: HashMap<K1, usize>,
}

impl<K1, V> MultiKeyMap<K1, V>
where
    K1: Eq + std::hash::Hash + Clone,
{
    pub fn new() -> Self {
        MultiKeyMap {
            values: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, keys: impl IntoIterator<Item = K1>, value: V) {
        // 新しい値を `values` に追加し、そのインデックスを取得
        self.values.push(value);
        let index = self.values.len() - 1;

        // 各キーにインデックスを対応付け
        for key in keys {
            self.map.insert(key, index);
        }
    }

    pub fn get(&self, key: &K1) -> Option<&V> {
        // キーが存在すればインデックスから値を取得
        self.map.get(key).and_then(|&index| self.values.get(index))
    }
}