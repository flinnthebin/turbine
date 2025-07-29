mod cli;
mod fan;
mod hwmap;
mod ioctl;
mod usb;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, Report, set_profile};
use fan::{Fan, FanId, FanSpeed};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let fan = Fan::new()?;

    let _ = match cli.command {
        Commands::Profile { profile } => set_profile(&fan, profile),
        Commands::Report => {
            let report = Report::get(&fan)?;
            report.print();
            Ok(())
        }
    };
    Ok(())
}
