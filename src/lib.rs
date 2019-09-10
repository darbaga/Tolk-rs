#[cfg(target_os = "windows")]
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

    pub fn try_sapi(&self, v: bool) {
        unsafe {
            Tolk_TrySAPI(v);
        }
    }

    pub fn prefer_sapi(&self, v: bool) {
        unsafe {
            Tolk_PreferSAPI(v);
        }
    }

    pub fn has_speech(&self) -> bool {
        unsafe {
            Tolk_HasSpeech()
        }
    }

    pub fn has_braille(&self) -> bool {
        unsafe {
            Tolk_HasBraille()
        }
    }

    pub fn detect_screen_reader(&self) -> Option<String> {
        let screen_reader = unsafe {
            Tolk_DetectScreenReader()
        };
        if screen_reader.is_null() {
            None
        } else {
            Some(unsafe { string_from_wchar_t(screen_reader) })
        }
    }

    pub fn output<S: Into<String>>(&self, s: S, interrupt: bool) {
        unsafe {
            Tolk_Output(str_to_wchar_t(&s.into()), interrupt);
        }
    }

    pub fn speak<S: Into<String>>(&self, s: S, interrupt: bool) -> bool {
        unsafe {
            Tolk_Speak(str_to_wchar_t(&s.into()), interrupt)
        }
    }

    pub fn braille<S: Into<String>>(&self, s: S) -> bool {
        unsafe {
            Tolk_Braille(str_to_wchar_t(&s.into()))
        }
    }

    pub fn is_speaking(&self) -> bool {
        unsafe {
            Tolk_IsSpeaking()
        }
    }

    pub fn silence(&self) -> bool {
        unsafe {
            Tolk_Silence()
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
