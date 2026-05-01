use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "imgx")]
#[command(version = "1.0")]
#[command(about = "High-performance Rust image processor")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Grayscale {
        input: String,
        output: String,
    },

    Rotate {
        input: String,
        output: String,
        degree: u32,
    },

    Brighten {
        input: String,
        output: String,
        value: i32,
    },

    BatchGrayscale {
        folder: String,
    },
}
