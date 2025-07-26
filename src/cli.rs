use crate::fan::*;
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum FanProfile {
    Off,
    Low,
    Med,
    High,
    Max,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Profile {
        #[arg(value_enum)]
        profile: FanProfile,
    },
}

pub fn set_profile(fan: &Fan, profile: FanProfile) -> Result<()> {
    use crate::FanId::*;
    use crate::FanSpeed::*;
    match profile {
        FanProfile::Off => {
            fan.set_fan(Right, Off)?;
            fan.set_fan(Left, Off)?;
        }
        FanProfile::Low => {
            fan.set_fan(Right, Off)?;
            fan.set_fan(Left, Off)?;
            std::thread::sleep(std::time::Duration::from_secs(1));
            fan.set_fan(Right, Bios)?;
            fan.set_fan(Left, Low)?;
        }
        FanProfile::Med => {
            fan.set_fan(Left, Off)?;
            fan.set_fan(Right, Low)?;
        }
        FanProfile::High => {
            fan.set_fan(Left, Low)?;
            fan.set_fan(Right, Low)?;
        }
        FanProfile::Max => {
            fan.set_fan(Left, High)?;
            fan.set_fan(Right, High)?;
        }
    }
    Ok(())
}
