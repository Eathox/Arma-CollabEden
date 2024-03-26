use bimap::BiHashMap;
use serde::{Deserialize, Serialize};

/// Unique network entity id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NetEntityId(u32);

impl NetEntityId {
    /// Create new network entity id. Only intended for tests.
    #[cfg(debug_assertions)]
    #[inline]
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
}

impl std::fmt::Display for NetEntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "N:{}", self.0)
    }
}

/// Local entity id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LocalEntityId(u32);

impl LocalEntityId {
    /// Create new local entity id.
    #[inline]
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
}

impl std::fmt::Display for LocalEntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "L:{}", self.0)
    }
}

/// Helper struct for generating unique network entity ids.
pub struct NetIdGenerator(u32);

impl NetIdGenerator {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) -> NetEntityId {
        let id = NetEntityId(self.0);
        self.0 += 1;
        id
    }
}

/// Mapping between local and network entity ids.
pub struct EntityIdMap(BiHashMap<NetEntityId, LocalEntityId>);

impl EntityIdMap {
    pub fn new() -> Self {
        Self(BiHashMap::new())
    }

    pub fn add(&mut self, net_id: NetEntityId, local_id: LocalEntityId) {
        self.0.insert(net_id, local_id);
    }

    pub fn remove(&mut self, net_id: NetEntityId) {
        self.0.remove_by_left(&net_id);
    }

    pub fn get_local_id(&self, net_id: NetEntityId) -> Option<LocalEntityId> {
        self.0.get_by_left(&net_id).copied()
    }

    pub fn get_net_id(&self, local_id: LocalEntityId) -> Option<NetEntityId> {
        self.0.get_by_right(&local_id).copied()
    }
}
