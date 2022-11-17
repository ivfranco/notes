use windows::{core::PCSTR, Win32::Storage::FileSystem::GetVolumeInformationA};

pub fn get_volume_serial() -> u32 {
    let mut serial = 0u32;

    // # Safety
    //
    // `drive` is null terminated, `serial` is properly aligned and initialized u32.
    unsafe {
        let drive = PCSTR::from_raw(b"C:\\\0".as_ptr());
        GetVolumeInformationA(drive, None, Some(&mut serial), None, None, None);
    }

    serial
}

pub fn keygen(user_name: &str) -> u64 {
    const LOW_KEY: u32 = 0xB14AC01A;
    const HIGH_KEY: u32 = 0x8ED105C2;

    let volume = get_volume_serial();
    let name_hash = user_name
        .as_bytes()
        .iter()
        .fold(0u64, |sum, b| sum + ((*b as u64) << (b % 48))) as u32;

    let low_part = name_hash.wrapping_mul(volume).wrapping_sub(LOW_KEY);
    let high_part = name_hash.wrapping_mul(volume).wrapping_sub(HIGH_KEY);

    low_part as u64 | ((high_part as u64) << 32)
}
