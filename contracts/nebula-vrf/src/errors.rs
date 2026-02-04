use soroban_sdk::contracterror;

/// Errors that can occur during VRF operations inside the NebulaVRF contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VRFError {
    InvalidSignature = 1,
    CommitmentMismatch = 2,
    NoCommitmentFound = 3,
}

