#![cfg(windows)]
extern crate tolk_sys;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::slice::from_raw_parts;

use tolk_sys::*;

pub struct Tolk;

impl Tolk {
    pub fn new() -> Tolk {
        unsafe {
            Tolk_Load();
        }
        Tolk
    }

    pub fn detect_screen_reader(&self) -> String {
        unsafe {
            return string_from_wchar_t(Tolk_DetectScreenReader());
        }
    }

    pub fn output(&self, s: &str) {
        unsafe {
            Tolk_Output(str_to_wchar_t(s), true);
        }
    }

    pub fn speak (&self, s: &str) {
        if unsafe { Tolk_HasSpeech() } {
            unsafe {
                Tolk_Speak(str_to_wchar_t(s), true);
            }
        } else {
            // Fallback on self.output
            self.output(s)
        }
    }

    pub fn braille (&self, s: &str) {
        if unsafe { Tolk_HasBraille() } {
            unsafe {
                Tolk_Braille(str_to_wchar_t(s));
            }
        } else {
            // Fallback on self.output
            self.output(s);
        }
    }
}

impl Drop for Tolk {
    fn drop(&mut self) {
        unsafe { Tolk_Unload(); }
    }
}

fn str_to_wchar_t(s: &str) -> *const u16 {
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr()
}

unsafe fn string_from_wchar_t(orig: *const u16) -> String {
    let mut len = 0;

    while *orig.offset(len) != 0 {
        len += 1;
    }

    let sl = from_raw_parts(orig, len as usize);
    String::from_utf16(sl).unwrap()
}

#[test]
fn test_drop_behavior() {
    unsafe {
        let f = Tolk::new();
        assert_eq!(Tolk_IsLoaded(), true);
    }

    unsafe { assert_eq!(Tolk_IsLoaded(), false); }
}