use super::NetEntityId;

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
