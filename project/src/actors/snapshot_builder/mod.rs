// Alias monádicos locales
pub type Domain   = crate::core::TranslateDomain;
pub type Codomain = crate::core::Snapshot;

pub mod snapshot_builder;
pub use snapshot_builder::SnapshotBuilder;
