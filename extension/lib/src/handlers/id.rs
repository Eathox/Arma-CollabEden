/// Unique network entity id.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    arma_rs::IntoArma,
    arma_rs::FromArma,
)]
pub struct NetEntityId(u32);

impl NetEntityId {
    /// Unique ID of the network entity.
    #[inline]
    #[must_use]
    pub const fn id(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for NetEntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Net:{}", self.id())
    }
}

pub struct NetEntityIdGen(u32);

impl NetEntityIdGen {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) -> NetEntityId {
        let id = NetEntityId(self.0);
        self.0 += 1;
        id
    }
}
