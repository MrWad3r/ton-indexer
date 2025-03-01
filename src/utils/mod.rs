use std::collections::{HashMap, HashSet};

pub use archive_package::*;
pub use block::*;
pub use block_proof::*;
pub use mapped_file::*;
pub use operations_pool::*;
pub use package_entry_id::*;
pub use progress_bar::*;
pub use shard_state::*;
pub use shard_state_cache::*;
pub use stored_value::*;
pub use top_blocks::*;
pub use with_archive_data::*;

mod archive_package;
mod block;
mod block_proof;
mod mapped_file;
mod operations_pool;
mod package_entry_id;
mod progress_bar;
mod shard_state;
mod shard_state_cache;
mod stored_value;
mod top_blocks;
mod with_archive_data;

pub(crate) type FastHashSet<K> = HashSet<K, FastHasherState>;
pub(crate) type FastHashMap<K, V> = HashMap<K, V, FastHasherState>;
pub(crate) type FastDashSet<K> = dashmap::DashSet<K, FastHasherState>;
pub(crate) type FastDashMap<K, V> = dashmap::DashMap<K, V, FastHasherState>;
pub(crate) type FastHasherState = ahash::RandomState;

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub requests: u64,
    pub occupied: u64,
    pub hits_ratio: f64,
    pub size_bytes: u64,
}
