mod cli;
mod fan;
mod ioctl;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, set_profile};
use fan::{Fan, FanId, FanSpeed};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let fan = Fan::new()?;

    let _ = match cli.command {
        Commands::Profile { profile } => set_profile(&fan, profile),
    };
    Ok(())
}
