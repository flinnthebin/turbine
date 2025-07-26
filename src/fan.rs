use crate::ioctl::*;
use anyhow::Result;
use std::{fs::File, os::fd::AsRawFd};

#[derive(Copy, Clone, Debug)]
pub enum FanId {
    Left,
    Right,
}

impl FanId {
    pub fn query(self) -> usize {
        match self {
            FanId::Left => 1,
            FanId::Right => 0,
        }
    }

    pub fn set(self) -> i32 {
        match self {
            FanId::Left => 1,
            FanId::Right => 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FanSpeed {
    Off,
    Low,
    High,
}

impl FanSpeed {
    pub fn set(self) -> i32 {
        match self {
            FanSpeed::Off => 0,
            FanSpeed::Low => 0,
            FanSpeed::High => 2,
        }
    }
}

pub struct Fan {
    fd: File,
}

impl Fan {
    pub fn new() -> Result<Self> {
        Ok(Self {
            fd: File::open("/proc/i8k")?,
        })
    }

    pub fn cpu_temp(&self) -> Result<usize> {
        let mut temp: usize = 0;
        unsafe {
            get_cpu_temp(self.fd.as_raw_fd(), &mut temp)?;
        }
        Ok(temp)
    }

    pub fn fan_speed(&self, fan: FanId) -> Result<usize> {
        let mut speed = fan.query();
        unsafe {
            get_fan_speed(self.fd.as_raw_fd(), &mut speed)?;
        }
        Ok(speed)
    }

    pub fn fan_status(&self, fan: FanId) -> Result<usize> {
        let mut status = fan.query();
        unsafe {
            get_fan_status(self.fd.as_raw_fd(), &mut status)?;
        }
        Ok(status)
    }

    pub fn set_fan(&self, fan: FanId, speed: FanSpeed) -> Result<()> {
        let mut args = [fan.set(), speed.set()];
        unsafe {
            set_fan_speed(self.fd.as_raw_fd(), &mut args)?;
        }
        Ok(())
    }
}
