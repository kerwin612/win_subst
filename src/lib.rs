extern crate windows;
use windows::{
    core::{PCWSTR, HSTRING},
    Win32::Storage::FileSystem::{DEFINE_DOS_DEVICE_FLAGS, DefineDosDeviceW},
};

pub fn add(link: &str, target: &str) -> bool {
    unsafe {
        return DefineDosDeviceW(DEFINE_DOS_DEVICE_FLAGS(0), &HSTRING::from(link), &HSTRING::from(target)).as_bool();
    }
}

pub fn del(link: &str) -> bool {
    unsafe {
        return DefineDosDeviceW(DEFINE_DOS_DEVICE_FLAGS(0x2), &HSTRING::from(link), PCWSTR::null()).as_bool();
    }
}

#[cfg(test)]
mod it_works {
    use super::*;
    use std::env;
    use std::path::Path;

    #[test]
    fn test_for_add() {
        assert_eq!(add("T:", env::current_dir().unwrap().as_path().to_str().unwrap()), true);
        assert_eq!(Path::new("T:").exists(), true);
    }

    #[test]
    fn test_for_del() {
        assert_eq!(del("T:"), true);
        assert_eq!(Path::new("T:").exists(), false);
    }
}
