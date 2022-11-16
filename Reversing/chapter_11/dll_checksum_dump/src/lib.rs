use std::{
    ffi::OsString,
    fs::File,
    io::{self, Read},
    mem,
    os::windows::prelude::OsStringExt,
    path::PathBuf,
    slice,
};

use goblin::pe::{options::ParseOptions, PE};
use ntapi::{
    ntpsapi::NtCurrentPeb,
    ntrtl::RtlGetNtSystemRoot,
    winapi::shared::ntdef::{PVOID, ULONG, UNICODE_STRING, WCHAR},
};

fn checksum(str: &[u8]) -> i32 {
    str.iter().enumerate().rfold(0, |csum, (i, &b)| {
        let sb = b as i32;
        csum.wrapping_add(sb << (i % 24))
    })
}

#[allow(dead_code)]
fn ntdll_image() -> &'static [u8] {
    // # Safety
    //
    // Honestly I have no idea. ntapi didn't specify safety requirements for any of its exported
    // API. At least I'm not modifying data behind the points.
    // The entire unsafe block can be replaced by reading %SYSTEMROOT%\SysWOW64\ntdll.dll into
    // memory, it's written as is as an exercise.
    //
    // Lifetime of return slice is 'static since it is loaded as long as the program is loaded.
    unsafe {
        let peb = *NtCurrentPeb();
        let ldr = *peb.Ldr;
        let list_entry = ldr.InLoadOrderModuleList;
        let mut flink = list_entry.Flink;
        flink = (*flink).Flink;
        let dllbase = *((flink as usize + 0x18) as *const PVOID);
        let size_of_image = *((flink as usize + 0x20) as *const ULONG);
        let name = *((flink as usize + 0x2C) as *const UNICODE_STRING);
        // UNICODE_STRING.Length is in bytes
        let name_slice =
            slice::from_raw_parts(name.Buffer, name.Length as usize / mem::size_of::<WCHAR>());

        assert_eq!(
            "ntdll.dll",
            String::from_utf16(name_slice).expect("valid utf16")
        );

        slice::from_raw_parts(dllbase as *const u8, size_of_image as usize)
    }
}

fn sys_dll_image(dll_name: &str) -> Result<Vec<u8>, io::Error> {
    // # Safety
    //
    // As RtlGenNtSystemRoot doesn't return an UNICODE_STRING structure with string length and
    // buffer length, it must be null terminated utf16.
    let system_root = unsafe {
        let wstr = RtlGetNtSystemRoot();
        let len = {
            let mut len = 0;
            let mut p = wstr;
            while *p != 0 {
                len += 1;
                p = p.add(1);
            }
            len
        };

        OsString::from_wide(slice::from_raw_parts(wstr, len))
    };

    let mut path = PathBuf::from(system_root);
    if path.join("SysWOW64").try_exists()? {
        // 64-bit Windows
        path.push("SysWOW64");
    } else {
        // 32-bit Windows
        path.push("System32");
    }
    path.push(dll_name);

    let mut file = File::open(&path)?;
    let mut image = Vec::with_capacity(file.metadata()?.len() as usize);
    file.read_to_end(&mut image)?;

    Ok(image)
}

// when reading directly from PEB, `resolve_rva` should be set to false as RVAs are already resolved
fn export_list(image: &[u8], resolve_rva: bool) -> Vec<(&str, usize)> {
    let opt = ParseOptions { resolve_rva };
    let pe = PE::parse_with_opts(image, &opt).expect("valid PE32 dll image");

    pe.exports
        .into_iter()
        .map(|e| {
            let name = e.name.unwrap_or("UNNAMED_EXPORT");
            let rva = e.rva;

            (name, rva)
        })
        .collect()
}

pub fn dump_sys_dll(dll_name: &str) -> Result<(), io::Error> {
    let image = sys_dll_image(dll_name)?;
    println!("{:<8}, {:<10}: export name", "checksum", "RVA");
    for (name, rva) in export_list(&image, true) {
        let csum = checksum(name.as_bytes());
        println!("{:0>8X}, 0x{:0>8X}: {}", csum, rva, name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_examples() {
        assert_eq!(checksum(b"NtAllocateVirtualMemory"), 0x39DBA17A);
        assert_eq!(checksum(b"ZwDelayExecution"), 0x006DEF20);
    }

    #[test]
    fn peb_black_magic() {
        let image = ntdll_image();
        let pe = PE::parse_with_opts(image, &ParseOptions { resolve_rva: false })
            .expect("valid ntdll image");
        assert_eq!(pe.name, Some("ntdll.dll"));
    }

    #[test]
    fn check_sys_dll() {
        assert!(sys_dll_image("ntdll.dll").is_ok());
        assert!(sys_dll_image("kernel32.dll").is_ok());
    }
}
