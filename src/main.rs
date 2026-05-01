mod cli;
mod filters;
mod processor;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use filters::{brighten::Brighten, grayscale::Grayscale, rotate::Rotate};
use processor::{process_batch, process_single};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Grayscale { input, output } => {
            process_single(&input, &output, Grayscale)?;
        }

        Commands::Rotate {
            input,
            output,
            degree,
        } => {
            process_single(&input, &output, Rotate { degree })?;
        }

        Commands::Brighten {
            input,
            output,
            value,
        } => {
            process_single(&input, &output, Brighten { value })?;
        }

        Commands::BatchGrayscale { folder } => {
            process_batch(folder, Grayscale)?;
        }
    }

    Ok(())
}
