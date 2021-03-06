use crate::{Error, MergeOperator, Toboggan, Tree};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

#[derive(Default, Clone)]
pub struct BTreeAdapter {
    trees: Arc<Mutex<BTreeMap<Vec<u8>, MemoryStoreTree>>>,
}

impl BTreeAdapter {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Clone)]
pub struct MemoryStoreTree {
    inner: Arc<Mutex<TreeInner>>,
}

#[derive(Default)]
struct TreeInner {
    btree: BTreeMap<Vec<u8>, Vec<u8>>,
    merge_operator: Option<Box<dyn MergeOperator>>,
}

impl Toboggan for BTreeAdapter {
    type Tree = MemoryStoreTree;

    fn open_tree<V: AsRef<[u8]>>(&self, name: V) -> Result<Self::Tree, crate::Error> {
        let mut trees = self.trees.lock().unwrap();
        match trees.entry(name.as_ref().to_owned()) {
            std::collections::btree_map::Entry::Vacant(v) => {
                Ok(v.insert(Default::default()).clone())
            }
            std::collections::btree_map::Entry::Occupied(o) => Ok(o.get().clone()),
        }
    }
}

impl Tree for MemoryStoreTree {
    type OutValue = Vec<u8>;
    type Iter = StupidIterator<(Self::OutValue, Self::OutValue)>;

    fn insert<K: AsRef<[u8]> + Into<Vec<u8>>, V: AsRef<[u8]>>(
        &self,
        key: K,
        value: V,
    ) -> Result<(), crate::Error> {
        self.inner
            .lock()
            .unwrap()
            .btree
            .insert(key.into(), value.as_ref().into());
        Ok(())
    }

    fn set_merge_operator(&self, merge_operator: impl crate::MergeOperator + 'static) {
        self.inner.lock().unwrap().merge_operator = Some(Box::new(merge_operator));
    }

    fn merge<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), crate::Error> {
        let mut inner = self.inner.lock().unwrap();

        // consider adding + Into<Vec<u8>>
        let key = key.as_ref();
        let value = value.as_ref();

        match &inner.merge_operator {
            Some(op) => {
                // entry API requires an unnecessary clone >_>
                let last = inner.btree.get(key).map(|v| v.as_ref());

                match op(key, last, value.as_ref()) {
                    Some(merged_value) => inner.btree.insert(key.to_vec(), merged_value),
                    None => inner.btree.remove(key),
                };
            }
            None => {
                inner.btree.insert(key.to_vec(), value.to_vec());
            }
        }

        Ok(())
    }

    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Self::OutValue>, crate::Error> {
        let inner = self.inner.lock().unwrap();
        Ok(inner.btree.get(key.as_ref()).map(|v| v.to_owned()))
    }

    fn flush(&self) -> Result<(), crate::Error> {
        Ok(())
    }

    fn clear(&self) -> Result<(), crate::Error> {
        let mut inner = self.inner.lock().unwrap();
        inner.btree.clear();
        Ok(())
    }

    fn iter(&self) -> StupidIterator<(Self::OutValue, Self::OutValue)> {
        let inner = self.inner.lock().unwrap();
        StupidIterator {
            list: inner
                .btree
                .iter()
                .map(|v| (v.0.to_owned(), v.1.to_owned()))
                .collect(),
            offset: 0,
        }
    }
}

pub struct StupidIterator<T> {
    offset: usize,
    list: Vec<T>,
}

impl<T> Iterator for StupidIterator<T>
where
    T: Clone,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list.get(self.offset) {
            Some(v) => {
                self.offset += 1;
                Some(Ok(v.clone()))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BTreeAdapter;
    use crate::tester::Tester;

    #[test]
    fn memory() {
        let store = BTreeAdapter::new();
        let tester = Tester(store);
        tester.test()
    }
}
