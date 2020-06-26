pub trait Repository<K, V> {
    fn has(&self, key: &K) -> bool;

    fn get(&self, key: &K) -> Option<V>;

    fn require(&self, key: &K) -> V;
}
