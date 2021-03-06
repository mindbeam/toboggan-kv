use crate::{Error, Toboggan, Tree};
#[derive(Clone)]
pub struct SledAdapter {
    db: sled::Db,
}
pub struct SledStoreTree(sled::Tree);

impl Toboggan for SledAdapter {
    type Tree = SledStoreTree;
    fn open_tree<V: AsRef<[u8]>>(&self, name: V) -> Result<Self::Tree, Error> {
        Ok(SledStoreTree(self.db.open_tree(name)?))
    }
}

impl Tree for SledStoreTree {
    type OutValue = sled::IVec;
    type Iter = SledIterWrapper;

    fn insert<K: AsRef<[u8]> + Into<Vec<u8>>, V: AsRef<[u8]>>(
        &self,
        key: K,
        value: V,
    ) -> Result<(), Error> {
        self.0.insert(key, value.as_ref())?;
        Ok(())
    }

    fn set_merge_operator(&self, merge_operator: impl crate::MergeOperator + 'static) {
        self.0.set_merge_operator(merge_operator);
    }

    fn merge<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error> {
        self.0.merge(key, value)?;
        Ok(())
    }

    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Self::OutValue>, Error> {
        Ok(self.0.get(key)?)
    }

    fn flush(&self) -> Result<(), Error> {
        self.0.flush()?;
        Ok(())
    }

    fn clear(&self) -> Result<(), Error> {
        self.0.clear()?;
        Ok(())
    }

    fn iter(&self) -> Self::Iter {
        SledIterWrapper(self.0.iter())
    }
}

impl SledAdapter {
    pub fn open_temp() -> Result<Self, Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();

        Self::open(tmpdirpath)
    }

    #[allow(dead_code)]
    pub fn open(basedir: &std::path::Path) -> Result<Self, Error> {
        let pathbuf = basedir.join(format!("./mindbase.sled"));

        let db = sled::open(pathbuf.as_path())?;

        Ok(SledAdapter { db })
    }
}

pub struct SledIterWrapper(sled::Iter);

impl Iterator for SledIterWrapper {
    type Item = Result<(sled::IVec, sled::IVec), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|o| o.map_err(|e| e.into()))
    }
}

#[cfg(test)]
mod test {
    use crate::{adapter::SledAdapter, tester::Tester};

    #[test]
    fn sled() {
        let store = SledAdapter::open_temp().unwrap();
        let tester = Tester(store);
        tester.test()
    }
}
