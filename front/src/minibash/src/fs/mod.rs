pub mod filesystem;
pub mod index;
#[macro_use]
pub mod macros;

pub mod store;

pub use filesystem::{MiniFs, RamFs};
pub use store::{Entries, Entry, EntryKind, Store, StoreExt};

pub mod prelude {
    pub use crate::fs::store::{Store, StoreExt};
}
