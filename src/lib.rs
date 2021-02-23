//! Toboggan - sled flavored KV abstraction
//! ```
//! # use toboggan_kv::{Toboggan,Tree,adapter::{BTreeAdapter, SledAdapter}};
//! let bta = BTreeAdapter::new();
//! braap(bta);
//!
//! let tmpdir = tempfile::tempdir().unwrap();
//! let sa = SledAdapter::open(tmpdir.path()).unwrap();
//! braap(sa);
//!
//! fn braap<T:Toboggan>( store: T ){
//!     let beasts = store.open_tree("beasts").unwrap();
//!     beasts.insert("meow", "cat").unwrap();
//! }
//! ```
//!

pub mod adapter;
pub mod error;

#[cfg(test)]
pub mod tester;

pub use error::Error;
use std::fmt::Debug;

pub trait Toboggan: Clone {
    type Tree: self::Tree;
    fn open_tree<V: AsRef<[u8]>>(&self, name: V) -> Result<Self::Tree, Error>;
}
pub trait MergeOperator: Fn(&[u8], Option<&[u8]>, &[u8]) -> Option<Vec<u8>> {}
impl<F> MergeOperator for F where F: Fn(&[u8], Option<&[u8]>, &[u8]) -> Option<Vec<u8>> {}

pub trait Tree {
    type OutValue: AsRef<[u8]>
        + std::borrow::Borrow<[u8]>
        + PartialEq<Vec<u8>>
        + std::ops::Deref<Target = [u8]>
        + Debug;
    type Iter: Iterator<Item = Result<(Self::OutValue, Self::OutValue), Error>>;

    fn insert<K: AsRef<[u8]> + Into<Vec<u8>>, V: AsRef<[u8]>>(
        &self,
        key: K,
        value: V,
    ) -> Result<(), Error>;
    fn set_merge_operator(&self, merge_operator: impl MergeOperator + 'static);
    fn merge<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error>;
    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Self::OutValue>, Error>;
    fn iter(&self) -> Self::Iter;
    fn flush(&self) -> Result<(), Error>;
    fn clear(&self) -> Result<(), Error>;
}
