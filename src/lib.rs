#[cfg(target_os = "windows")]
use tolk_sys::*;
use widestring::U16CString;

#[derive(Clone, Debug)]
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
        unsafe { Tolk_HasSpeech() }
    }

    pub fn has_braille(&self) -> bool {
        unsafe { Tolk_HasBraille() }
    }

    pub fn output<S: Into<String>>(&self, s: S, interrupt: bool) {
        let s = s.into();
        let s = U16CString::from_str(s);
        if let Some(s) = s.ok() {
            unsafe {
                Tolk_Output(s.as_ptr(), interrupt);
            }
        }
    }

    pub fn speak<S: Into<String>>(&self, s: S, interrupt: bool) -> bool {
        let s = s.into();
        let s = U16CString::from_str(s);
        if let Some(s) = s.ok() {
            unsafe { Tolk_Speak(s.as_ptr(), interrupt) }
        } else {
            false
        }
    }

    pub fn braille<S: Into<String>>(&self, s: S) -> bool {
        let s = s.into();
        let s = U16CString::from_str(s);
        if let Some(s) = s.ok() {
            unsafe { Tolk_Braille(s.as_ptr()) }
        } else {
            false
        }
    }

    pub fn is_speaking(&self) -> bool {
        unsafe { Tolk_IsSpeaking() }
    }

    pub fn silence(&self) -> bool {
        unsafe { Tolk_Silence() }
    }

    pub fn detect_screen_reader() -> Option<String> {
        let screen_reader = unsafe { Tolk_DetectScreenReader() };
        if screen_reader.is_null() {
            None
        } else {
            let screen_reader = unsafe { U16CString::from_ptr_with_nul(screen_reader, 64) };
            if let Some(screen_reader) = screen_reader.ok() {
                screen_reader.to_string().ok()
            } else {
                None
            }
        }
    }
}

impl Drop for Tolk {
    fn drop(&mut self) {
        unsafe {
            Tolk_Unload();
        }
    }
}

#[test]
fn test_drop_behavior() {
    unsafe {
        let f = Tolk::new();
        assert_eq!(Tolk_IsLoaded(), true);
    }

    unsafe {
        assert_eq!(Tolk_IsLoaded(), false);
    }
}
