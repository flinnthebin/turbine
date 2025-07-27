use crate::fan::*;
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

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
    Report,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum FanProfile {
    Off,
    Low,
    Med,
    High,
    Max,
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

pub struct Report {
    pub cpu_temp: usize,
    pub left_mode: usize,
    pub left_speed: usize,
    pub right_mode: usize,
    pub right_speed: usize,
}

impl Report {
    pub fn get(fan: &Fan) -> Result<Self> {
        Ok(Self {
            cpu_temp: fan.cpu_temp()?,
            left_mode: fan.fan_status(FanId::Left)?,
            left_speed: fan.fan_speed(FanId::Left)?,
            right_mode: fan.fan_status(FanId::Right)?,
            right_speed: fan.fan_speed(FanId::Right)?,
        })
    }

    pub fn print(&self) {
        println!(
            "CPU Temperature  (°C): {}\n\
             Left Fan Mode   (0-2): {}\n\
             Left Fan Speed  (rpm): {}\n\
             Right Fan Mode  (0-2): {}\n\
             Right Fan Speed (rpm): {}",
            self.cpu_temp, self.left_mode, self.left_speed, self.right_mode, self.right_speed,
        );
    }
}
