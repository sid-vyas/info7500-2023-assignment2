use clap::{Parser, Subcommand};

mod prover;
mod verifier;
mod util;

#[derive(Parser)]
struct Cli {
   #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Prove { num_leaves: usize, leaf_pos: usize,  },
    Verify { proof_file: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Prove { leaf_pos, num_leaves } => {
            prover::run(*leaf_pos, *num_leaves);
        }
        Commands::Verify { proof_file } => {
            verifier::run(proof_file)
        }
    }
}
