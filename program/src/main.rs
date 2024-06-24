#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{private::FixedBytes, sol, SolValue};
use ibc_core_commitment_types::{
    merkle::MerkleProof,
    proto::{
        ics23::HostFunctionsManager,
        v1::{MerklePath, MerkleProof as RawMerkleProof, MerkleRoot},
    },
    specs::ProofSpecs,
};
use prost::Message;

sol! {
    struct ICS23Output {
        bytes32 appHash;
        // NOTE: This is simply a proof of concept, and ideally only critical
        // information from the commitment path and value are outputted.
        string keyPath;
    }
}

pub fn main() {
    // INPUTS

    let encoded_1 = sp1_zkvm::io::read_vec();
    let app_hash: [u8; 32] = encoded_1.clone().try_into().unwrap();
    let merkle_root = MerkleRoot { hash: encoded_1 };

    let encoded_2 = sp1_zkvm::io::read_vec();
    let raw_merkle_proof = RawMerkleProof::decode(&*encoded_2).unwrap();
    let merkle_proof = MerkleProof::try_from(raw_merkle_proof).unwrap();

    let encoded_3 = sp1_zkvm::io::read_vec();
    let merkle_path = MerklePath::decode(&*encoded_3).unwrap();
    let key_path = merkle_path.key_path.join("/");

    let commitment_value = sp1_zkvm::io::read_vec();

    // LOGIC

    let result = merkle_proof.verify_membership::<HostFunctionsManager>(
        &ProofSpecs::cosmos(),
        merkle_root,
        merkle_path,
        commitment_value,
        0,
    );

    match result {
        Ok(_) => println!("verified ics23 membership"),
        Err(error) => panic!("expected success, got: {:?}", error),
    }

    // OUTPUTS

    let output = ICS23Output {
        appHash: FixedBytes::from(app_hash),
        keyPath: key_path,
    };

    sp1_zkvm::io::commit_slice(&output.abi_encode());
}
