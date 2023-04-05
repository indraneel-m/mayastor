use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NvmeSnapshotMessageV1 {
    txn_id: String,
    parent_id: String,
    entity_id: String,
    name: String,
}

impl NvmeSnapshotMessageV1 {
    pub fn new(txn_id: String, parent_id: String, entity_id: String, name: String) -> Self {
        Self {
            txn_id,
            parent_id,
            entity_id,
            name,
        }
    }

    pub fn txn_id(&self) -> &str {
        &self.txn_id
    }

    pub fn parent_id(&self) -> &str {
        &self.parent_id
    }

    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NvmeSnapshotMessage {
    V1(NvmeSnapshotMessageV1),
}