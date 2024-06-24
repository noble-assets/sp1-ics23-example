use base64::{engine::general_purpose, Engine};
use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};

pub const ICS23_ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ProveArgs {
    #[clap()]
    merkle_root: String,

    #[clap()]
    merkle_proof: String,

    #[clap()]
    merkle_path: String,

    #[clap()]
    commitment_value: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = ProveArgs::parse();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the program.
    let (pk, vk) = client.setup(ICS23_ELF);

    // Setup the inputs.;
    let mut stdin = SP1Stdin::new();

    stdin.write_vec(general_purpose::STANDARD.decode(args.merkle_root).unwrap());
    stdin.write_vec(general_purpose::STANDARD.decode(args.merkle_proof).unwrap());
    stdin.write_vec(general_purpose::STANDARD.decode(args.merkle_path).unwrap());
    stdin.write_vec(
        general_purpose::STANDARD
            .decode(args.commitment_value)
            .unwrap(),
    );

    // Generate the proof.
    let proof = client.prove(&pk, stdin).expect("failed to generate proof");
    println!("Successfully generated ICS23 proof!");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
}
