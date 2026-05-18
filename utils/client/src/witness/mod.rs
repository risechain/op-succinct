pub mod executor;
pub mod preimage_store;

use std::fmt::Debug;

use kzg_rs::{Blob, Bytes48};
use serde::{Deserialize, Serialize};

use crate::witness::preimage_store::PreimageStore;

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct BlobData {
    pub blobs: Vec<Blob>,
    pub commitments: Vec<Bytes48>,
    pub proofs: Vec<Bytes48>,
}

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct DefaultWitnessData {
    pub preimage_store: PreimageStore,
    pub blob_data: BlobData,
}
