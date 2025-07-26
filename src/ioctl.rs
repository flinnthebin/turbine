use nix::{ioctl_read, ioctl_readwrite};

ioctl_read!(get_cpu_temp, b'i', 0x84, usize);
ioctl_readwrite!(get_fan_speed, b'i', 0x85, usize);
ioctl_readwrite!(get_fan_status, b'i', 0x86, usize);
ioctl_readwrite!(set_fan_speed, b'i', 0x87, [i32; 2]);

#[cfg(test)]
mod tests {
    #[test]
    fn test_read() {
        println!(
            "Rust IOCTL req: {:?}",
            nix::request_code_read!(b'i', 0x84, std::mem::size_of::<i32>())
        );
    }
    #[test]
    fn test_readwrite() {
        println!(
            "Rust IOCTL req: {:?}",
            nix::request_code_readwrite!(b'i', 0x87, std::mem::size_of::<[i32; 2]>())
        );
    }
}
