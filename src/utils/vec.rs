///Returns true if `vec` contains `key`
pub fn contains_key<K, V>(vec: &Vec<(K, V)>, key: &K) -> bool
where
    K: PartialEq,
{
    for (k, _) in vec {
        if k == key {
            return true;
        }
    }
    false
}

///Returns a copy of the value associated with `key`
pub fn get<'a, K, V>(vec: &'a Vec<(K, V)>, key: &K) -> Option<&'a V>
where
    K: PartialEq,
{
    for (k, v) in vec {
        if k == key {
            return Some(v);
        }
    }
    None
}

#[test]
fn test_contains_key() {
    let hash_map = vec![("apple".to_string(), 1), ("pear".to_string(), 2)];
    let contains = contains_key(&hash_map, &"apple".to_string());

    assert!(contains);
}

#[test]
fn test_get() {
    let hash_map = vec![("apple".to_string(), 1), ("pear".to_string(), 2)];
    let value = get(&hash_map, &"apple".to_string()).unwrap();

    assert_eq!(*value, 1);
}
