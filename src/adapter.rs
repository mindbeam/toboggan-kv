pub mod btree;

pub use btree::BTreeAdapter;

#[cfg(not(target_arch = "wasm32"))]
pub mod sled;

#[cfg(not(target_arch = "wasm32"))]
pub use self::sled::SledAdapter;
