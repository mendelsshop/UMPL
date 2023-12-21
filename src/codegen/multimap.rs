use std::ptr::NonNull;

use std::collections::HashMap;

use std::hash::Hash;

/// a `HashMap` like thing that has two types of indexes
/// 1) list of indices that can get the value
/// 2) a single index that can set the value
// SAFETY:
// 1) All the NonNull stored in the map are garunteed to be vaild because they are obtained from [Box::into_raw].
// 2) The get and set methods with rust ownership rules garuntee that NonNulls are either have a lot of shared references
// or a single exlusive reference
// in other words this map can be implemented without the types that are generally used for interior mutablity and without unsafe
// by just having the keys map be a map of key to key and then get would first lookup in keys and then the key from keys looks up in the map for values
pub struct MultiMap<K: Hash + Eq, V> {
    pub(crate) keys: HashMap<K, NonNull<V>>,
    pub(crate) values: HashMap<K, NonNull<V>>,
}

impl<K: Hash + Eq, V> MultiMap<K, V> {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            values: HashMap::new(),
        }
    }
    /// allows you te get from on of the multiple keys
    pub fn get(&self, key: &K) -> Option<&V> {
        // SAFETY: were allowed to obtain a shared reference to the value because you can only obtain an exclusive reference to the value
        // if you call set, which we requires an exclusive reference to the MultiMap, which means you cannot also obtain shared references to the value
        // so in short the method signatures of the MultiMap garuntee the safety of using unsafe
        self.keys.get(key).map(|v| unsafe { v.as_ref() })
    }

    /// allows you to mutate a value based on its mutatable key
    pub fn set(&mut self, key: &K, setter: impl FnOnce(&V) -> V) -> Option<()> {
        // SAFETY: were allowed to obtain an exlusive reference to the value because you can only obtain an exclusive reference to the value
        // if you call set, which we requires an exclusive reference to the MultiMap, which means you cannot also obtain shared references to the value or another exlusive reference to the value
        // so in short the method signatures of the MultiMap garuntee the safety of using unsafe
        self.values.get(key).map(|v| unsafe {
            *v.as_ptr() = setter(&*v.as_ptr());
        })
    }

    pub fn get_or_set(
        &mut self,
        key: &K,
        getter: impl FnOnce(&V),
        setter: impl FnOnce(&V) -> V,
    ) -> Option<()> {
        self.get(key)
            .map(|v| {
                getter(v);
            })
            .or_else(|| self.set(key, setter))
    }
}

impl<K: Hash + Eq, V> Drop for MultiMap<K, V> {
    fn drop(&mut self) {
        self.values
            .iter_mut()
            // SAFETY: see from impl
            .for_each(|(_, v)| drop(unsafe { Box::from_raw(v.as_ptr()) }));
    }
}

impl<T: IntoIterator<Item = (KS, K, V)>, K: Hash + Eq + Clone, V, KS: IntoIterator<Item = K>>
    From<T> for MultiMap<K, V>
{
    fn from(value: T) -> Self {
        let (values, keys): (HashMap<K, _>, Vec<_>) = value
            .into_iter()
            .map(|(keys, key, value): (KS, K, V)| {
                let value = Box::into_raw(Box::new(value));
                (
                    // SAFTEY: Box::from_raw does not give us a null pointer
                    (key, unsafe { NonNull::new_unchecked(value) }),
                    keys.into_iter()
                        // SAFTEY: Box::from_raw does not give us a null pointer
                        .map(|keys_outer: K| (keys_outer, unsafe { NonNull::new_unchecked(value) }))
                        .collect::<Vec<_>>(),
                )
            })
            .unzip();

        let keys: HashMap<_, _> = keys.into_iter().flatten().collect();
        Self { keys, values }
    }
}
