mod fan;
mod ioctl;

use anyhow::Result;
use fan::{Fan, FanId, FanSpeed};
use std::{thread, time};

fn main() -> Result<()> {
    let fan = Fan::new()?;
    println!("CPU Temperature  (°C): {}", fan.cpu_temp()?);
    println!("Left Fan Mode   (0-2): {}", fan.fan_status(FanId::Left)?);
    println!("Left Fan Speed  (rpm): {}", fan.fan_speed(FanId::Left)?);
    println!("Right Fan Mode  (0-2): {}", fan.fan_status(FanId::Right)?);
    println!("Right Fan Speed (rpm): {}", fan.fan_speed(FanId::Right)?);

    // Cycle through all states for both fans
    let steps = [
        (FanId::Left, FanSpeed::Off),
        (FanId::Right, FanSpeed::Off),
        (FanId::Left, FanSpeed::Low),
        (FanId::Right, FanSpeed::Low),
        (FanId::Left, FanSpeed::High),
        (FanId::Right, FanSpeed::High),
    ];
    for (fan_id, speed) in steps {
        fan.set_fan(fan_id, speed)?;
        println!("{:?} Fan set to {:?}", fan_id, speed);
        println!(
            "Left  Mode: {} | Speed: {}",
            fan.fan_status(FanId::Left)?,
            fan.fan_speed(FanId::Left)?
        );
        println!(
            "Right Mode: {} | Speed: {}",
            fan.fan_status(FanId::Right)?,
            fan.fan_speed(FanId::Right)?
        );
        thread::sleep(time::Duration::from_secs(3));
    }
    Ok(())
}
