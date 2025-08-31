//! Dominio del actor VelloAdapter: lo que viene del Translator.
use crate::translator::Snapshot;

/// Dominio: salida del Translator (lista para adaptar a Vello).
pub type AdaptDomain = Snapshot;
