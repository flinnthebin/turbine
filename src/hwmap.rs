pub enum HardwareCode {
    Bios,
    SleepCharging,
    LightsOff,
    LightsOn,
    Full,
    Charging,
    SleepDischarging,
    On,
    Low,
}

impl HardwareCode {
    pub fn as_u16(self) -> u16 {
        match self {
            HardwareCode::Bios => 1,
            HardwareCode::SleepCharging => 2,
            HardwareCode::LightsOff => 3,
            HardwareCode::LightsOn => 4,
            HardwareCode::Full => 5,
            HardwareCode::Charging => 6,
            HardwareCode::SleepDischarging => 7,
            HardwareCode::On => 8,
            HardwareCode::Low => 9,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum KeyboardQuadrants {
    First,
    Second,
    Third,
    Fourth,
}

impl KeyboardQuadrants {
    pub fn as_u16(self) -> u16 {
        match self {
            KeyboardQuadrants::First => 1,
            KeyboardQuadrants::Second => 2,
            KeyboardQuadrants::Third => 3,
            KeyboardQuadrants::Fourth => 4,
        }
    }
}
