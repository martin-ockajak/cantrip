use std::collections::HashMap;
use std::hash::Hash;

// pub fn add_map<K, V>(values: &HashMap<K, V>, key: &K, value: &V) -> HashMap<K, V>
// where
//   K: Clone + Eq + Hash,
//   V: Clone,
// {
//   values.iter().map(|(k, v)| (k.clone(), v.clone())).chain([(key.clone(), value.clone())].into_iter()).collect()
// }

pub fn remove_all_map<K, V>(values: &HashMap<K, V>, keys: &[K]) -> HashMap<K, V>
  where
    K: Clone + Eq + Hash,
    V: Clone,
{
  values.iter().filter(|(k, _)| !keys.contains(k)).map(|(k, v)| (k.clone(), v.clone())).collect()
}

pub fn remove_map<K, V>(values: &HashMap<K, V>, key: &K) -> HashMap<K, V>
  where
    K: Clone + Eq + Hash,
    V: Clone,
{
  values.iter().filter(|(k, _)| k != &key).map(|(k, v)| (k.clone(), v.clone())).collect()
}

pub fn merge_map<K, V>(values1: &HashMap<K, V>, values2: &HashMap<K, V>) -> HashMap<K, V>
  where
    K: Clone + Eq + Hash,
    V: Clone,
{
  values1.iter().chain(values2.iter()).map(|(k, v)| (k.clone(), v.clone())).collect()
}

#[cfg(test)]
mod tests {

}
