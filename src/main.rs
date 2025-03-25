use std::str::FromStr;
use solana_program::pubkey::Pubkey;
use clap::Parser;

#[derive(Parser)]
#[clap(
    author = "fikunmi_ap",
    version,
    about = "A dead simple CLI tool to derive Solana PDAs from the progra ID and seeds. Returns the ID and seed.",
)]
struct Args{
    /// The Pubkey or ID from which the PDAs will be derived.
    #[clap(help = "Program ID (base-58 encoded public key)", required = true)]
    program_id: String,

    /// The seeds used to derive the PDA, make sure to enter them in the exact order.
    #[clap(help = "Seeds for PDA derivation (in order)", required = true, num_args = 1..=16)]
    seeds: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let program_id = Pubkey::from_str(&args.program_id)
        .unwrap_or_else(|err| {
            eprintln!("Error: Invalid Program ID: {}", err);
            std::process::exit(1001);
        });
    let seeds_vec: Vec<&[u8]> = args.seeds
        .iter()
        .map(|seed|seed.as_bytes())
        .collect();

    let (pda, bump) = Pubkey::find_program_address(&seeds_vec, &program_id);
    println!("Program ID: {}", program_id);
    println!("Seeds:");
    for (i, seed) in args.seeds.iter().enumerate() {
        println!("  {}: {}", i + 1, seed);
    }
    println!("\nDerived PDA: {}", pda);
    println!("Bump seed: {}", bump);
}