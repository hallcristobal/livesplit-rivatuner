extern crate kernel32;
extern crate winapi;

use winapi::{FILE_MAP_ALL_ACCESS, c_char};
use kernel32::{OpenFileMappingW, MapViewOfFile, UnmapViewOfFile, CloseHandle};
use std::ffi::{CStr, OsStr};
use std::os::windows::ffi::OsStrExt;

mod mem;

fn strlen(arr: &[c_char]) -> usize {
    arr.iter().take_while(|&&c| c != 0).count()
}

fn streq(a: &[c_char], b: &[c_char]) -> bool {
    for (&a, &b) in a.iter().zip(b.iter()) {
        if a != b {
            return false;
        }
        if a == 0 {
            return true;
        }
    }
    false
}

fn encode_wide(s: &str) -> Vec<u16> {
    let mut chars = OsStr::new(s).encode_wide().collect::<Vec<_>>();
    chars.push(0);
    chars
}

pub fn _print(text: &str) {
    unsafe {
        let mut text = text.as_bytes().iter().map(|&c| c as c_char).collect::<Vec<_>>();
        text.push(0);

        let name = encode_wide("RTSSSharedMemoryV2");
        let mut livesplit_text = "LiveSplit".bytes().map(|c| c as c_char).collect::<Vec<_>>();
        livesplit_text.push(0);

        let map_file = OpenFileMappingW(FILE_MAP_ALL_ACCESS, false as _, name.as_ptr());

        assert!(!map_file.is_null());

        let map_addr = MapViewOfFile(map_file, FILE_MAP_ALL_ACCESS, 0, 0, 0);

        assert!(!map_addr.is_null());

        let mem = &mut *(map_addr as *mut mem::SharedMemory);

        assert_eq!(mem.signature, 0x52545353);
        assert!(mem.version >= 0x00020000);

        'text: for pass in 0..2 {
            for entry in 1..mem.osd_arr_size {
                let entry = &mut *((mem as *mut mem::SharedMemory as *mut u8)
                    .offset(mem.osd_arr_offset as isize +
                            entry as isize * mem.osd_entry_size as isize) as
                                   *mut mem::OsdEntry);

                if pass > 0 {
                    if strlen(&entry.osd_owner) == 0 {
                        entry.osd_owner[..livesplit_text.len()].copy_from_slice(&livesplit_text);
                    }
                }

                if streq(&entry.osd_owner, &livesplit_text) {
                    if mem.version >= 0x00020007 {
                        entry.osd_ex[..text.len()].copy_from_slice(&text);
                    } else {
                        entry.osd[..text.len()].copy_from_slice(&text);
                    }

                    mem.osd_frame += 1;

                    break 'text;
                }
            }
        }

        UnmapViewOfFile(map_addr);
        CloseHandle(map_file);
    }
}

unsafe fn str(s: *const c_char) -> &'static str {
    if s.is_null() {
        ""
    } else {
        CStr::from_ptr(s as _).to_str().unwrap()
    }
}

#[no_mangle]
pub unsafe extern "C" fn print(s: *const c_char) {
    _print(str(s))
}
