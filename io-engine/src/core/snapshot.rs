use crate::lvs::Lvol;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
/// Snapshot Captures all the Snapshot information for Lvol.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SnapshotParams {
    entity_id: Option<String>,
    parent_id: Option<String>,
    txn_id: Option<String>,
    snap_name: Option<String>,
    snapshot_uuid: Option<String>,
    create_time: Option<String>,
}

/// Implement Snapshot Common Function.
impl SnapshotParams {
    pub fn new(
        entity_id: Option<String>,
        parent_id: Option<String>,
        txn_id: Option<String>,
        snap_name: Option<String>,
        snapshot_uuid: Option<String>,
        create_time: Option<String>,
    ) -> SnapshotParams {
        SnapshotParams {
            entity_id,
            parent_id,
            txn_id,
            snap_name,
            snapshot_uuid,
            create_time,
        }
    }
}
/// Parameters details for the Snapshot Clone.
#[derive(Clone, Debug)]
pub struct CloneParams {
    /// Clone replica name.
    pub clone_name: Option<String>,
    /// Clone replica uuid.
    pub clone_uuid: Option<String>,
    /// Source uuid from which the clone to be created.
    pub source_uuid: Option<String>,
    /// Timestamp when the clone is created.
    pub clone_create_time: Option<String>,
}
impl CloneParams {
    pub fn new(
        clone_name: Option<String>,
        clone_uuid: Option<String>,
        source_uuid: Option<String>,
        clone_create_time: Option<String>,
    ) -> Self {
        CloneParams {
            clone_name,
            clone_uuid,
            source_uuid,
            clone_create_time,
        }
    }
    /// Get clone name.
    pub fn clone_name(&self) -> Option<String> {
        self.clone_name.clone()
    }
    /// Set clone name.
    pub fn set_clone_name(&mut self, clone_name: String) {
        self.clone_name = Some(clone_name);
    }
    /// Get clone uuid.
    pub fn clone_uuid(&self) -> Option<String> {
        self.clone_uuid.clone()
    }
    /// Set clone uuid.
    pub fn set_clone_uuid(&mut self, clone_uuid: String) {
        self.clone_uuid = Some(clone_uuid);
    }
    /// Get source uuid from which clone is created.
    pub fn source_uuid(&self) -> Option<String> {
        self.source_uuid.clone()
    }
    /// Set source uuid.
    pub fn set_source_uuid(&mut self, uuid: String) {
        self.source_uuid = Some(uuid);
    }
    /// Get clone creation time.
    pub fn clone_create_time(&self) -> Option<String> {
        self.clone_create_time.clone()
    }
    /// Set clone create time.
    pub fn set_clone_create_time(&mut self, time: String) {
        self.clone_create_time = Some(time);
    }
}
/// Snapshot Descriptor to respond back as part of listsnapshot.
#[derive(Clone, Debug)]
pub struct VolumeSnapshotDescriptor {
    pub snapshot_lvol: Lvol,
    pub source_uuid: String,
    pub snapshot_size: u64,
    pub snap_param: SnapshotParams,
    pub num_clones: u64, /* TODO: Need to move to SnapshotParams part of
                          * clone feature. */
    // set to false, if any of the snapshotdescriptor is not filled properly
    pub valid_snapshot: bool,
}
impl VolumeSnapshotDescriptor {
    pub fn new(
        snapshot_lvol: Lvol,
        source_uuid: String,
        snapshot_size: u64,
        snap_param: SnapshotParams,
        num_clones: u64,
        valid_snapshot: bool,
    ) -> Self {
        Self {
            snapshot_lvol,
            source_uuid,
            snapshot_size,
            snap_param,
            num_clones,
            valid_snapshot,
        }
    }
    /// Get snapshot lvol.
    pub fn snapshot_lvol(&self) -> &Lvol {
        &self.snapshot_lvol
    }
    /// Get snapshot_uuid.
    pub fn source_uuid(&self) -> String {
        self.source_uuid.clone()
    }

    /// Give amount of bytes owned by snapshot.
    pub fn snapshot_size(&self) -> u64 {
        self.snapshot_size
    }

    /// Get SnapshotParameters.
    pub fn snapshot_params(&self) -> &SnapshotParams {
        &self.snap_param
    }

    /// Give number of clones.
    pub fn num_clones(&self) -> u64 {
        self.num_clones
    }

    /// Get ValidSnapshot value.
    pub fn valid_snapshot(&self) -> bool {
        self.valid_snapshot
    }
}

/// Snapshot attributes used to store its properties.
#[derive(Debug, EnumCountMacro, EnumIter)]
pub enum SnapshotXattrs {
    TxId,
    EntityId,
    ParentId,
    SnapshotUuid,
    SnapshotCreateTime,
}

impl SnapshotXattrs {
    pub fn name(&self) -> &'static str {
        match *self {
            Self::TxId => "io-engine.tx_id",
            Self::EntityId => "io-engine.entity_id",
            Self::ParentId => "io-engine.parent_id",
            Self::SnapshotUuid => "uuid",
            Self::SnapshotCreateTime => "io-engine.snapshot_create_time",
        }
    }
}
/// Clone attributes used to store its properties.
#[derive(Debug, EnumCountMacro, EnumIter)]
pub enum CloneXattrs {
    CloneUuid,
    SourceUuid,
    CloneCreateTime,
}
impl CloneXattrs {
    pub fn name(&self) -> &'static str {
        match *self {
            Self::CloneUuid => "uuid",
            Self::SourceUuid => "io-engine.source_uuid",
            Self::CloneCreateTime => "io-engine.clone_create_time",
        }
    }
}
///  Traits gives the common snapshot/clone interface for Local/Remote Lvol.
#[async_trait(?Send)]
pub trait SnapshotOps {
    type Error;
    type SnapshotIter;
    type Lvol;
    /// Create Snapshot Common API.
    async fn create_snapshot(
        &self,
        snap_param: SnapshotParams,
    ) -> Result<Lvol, Self::Error>;

    // Get a Snapshot Iterator.
    async fn snapshot_iter(self) -> Self::SnapshotIter;

    /// Prepare Snapshot Config for Block/Nvmf Device, before snapshot create.
    fn prepare_snap_config(
        &self,
        snap_name: &str,
        entity_id: &str,
        txn_id: &str,
        snap_uuid: &str,
    ) -> Option<SnapshotParams>;

    /// List Snapshot details based on source UUID from which snapshot is
    /// created.
    fn list_snapshot_by_source_uuid(&self) -> Vec<VolumeSnapshotDescriptor>;

    /// List Single snapshot details based on snapshot UUID.
    fn list_snapshot_by_snapshot_uuid(&self) -> Vec<VolumeSnapshotDescriptor>;

    async fn create_clone(
        &self,
        clone_param: CloneParams,
    ) -> Result<Self::Lvol, Self::Error>;

    /// Prepare clone config for snapshot.
    fn prepare_clone_config(
        &self,
        clone_name: &str,
        clone_uuid: &str,
        source_uuid: &str,
    ) -> Option<CloneParams>;

    /// Get clone count.
    fn snapshot_clone_count(&self) -> u64;
}

/// Traits gives the Snapshots Related Parameters.
pub trait SnapshotDescriptor {
    /// Get Transaction Id of the Snapshot Create.
    fn txn_id(&self) -> Option<String>;

    /// Set Transaction Id.
    fn set_txn_id(&mut self, txn_id: String);

    /// Get Entity Id of the Snapshot.
    fn entity_id(&self) -> Option<String>;

    /// Set Entity Id.
    fn set_entity_id(&mut self, entity_id: String);

    /// Get Parent Id of the Snapshot.
    fn parent_id(&self) -> Option<String>;

    /// Set Parent id of the Snapshot.
    fn set_parent_id(&mut self, parent_id: String);

    /// Get Snapshot Name.
    fn name(&self) -> Option<String>;

    /// Set Snapshot Name.
    fn set_name(&mut self, name: String);

    /// Get snapshot uuid of the snapshot.
    fn snapshot_uuid(&self) -> Option<String>;

    /// Set snapshot uuid of the snapshot.
    fn set_snapshot_uuid(&mut self, snapshot_uuid: String);

    /// Get snapshot create time.
    fn create_time(&self) -> Option<String>;

    /// Set snapshot create time.
    fn set_create_time(&mut self, time: String);
}

/// Trait to give interface for all Snapshot Parameters.
impl SnapshotDescriptor for SnapshotParams {
    /// Get Trasanction Id of the Snapshot.
    fn txn_id(&self) -> Option<String> {
        self.txn_id.clone()
    }

    /// Set Transaction Id.
    fn set_txn_id(&mut self, txn_id: String) {
        self.txn_id = Some(txn_id);
    }
    /// Get Entity Id of the Snapshot.
    fn entity_id(&self) -> Option<String> {
        self.entity_id.clone()
    }

    /// Set Entity Id.
    fn set_entity_id(&mut self, entity_id: String) {
        self.entity_id = Some(entity_id);
    }

    /// Get Parent Id of the Snapshot.
    fn parent_id(&self) -> Option<String> {
        self.parent_id.clone()
    }

    /// Set Parent id of the Snapshot.
    fn set_parent_id(&mut self, parent_id: String) {
        self.parent_id = Some(parent_id)
    }
    /// Get Snapname of the Snapshot.
    fn name(&self) -> Option<String> {
        self.snap_name.clone()
    }
    /// Set Snapshot Name.
    fn set_name(&mut self, name: String) {
        self.snap_name = Some(name);
    }
    /// Get snapshot uuid of the snapshot.
    fn snapshot_uuid(&self) -> Option<String> {
        self.snapshot_uuid.clone()
    }
    /// Set snapshot uuid of the snapshot.
    fn set_snapshot_uuid(&mut self, snapshot_uuid: String) {
        self.snapshot_uuid = Some(snapshot_uuid);
    }
    /// Get snapshot create time.
    fn create_time(&self) -> Option<String> {
        self.create_time.clone()
    }

    /// Set snapshot create time.
    fn set_create_time(&mut self, time: String) {
        self.create_time = Some(time);
    }
}