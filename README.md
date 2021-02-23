# toboggan-kv

Sled-flavored KV abstraction layer

```
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
```