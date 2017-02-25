extern crate tolk_sys;

use tolk_sys::*;

struct Tolk;

impl Tolk {
    pub fn new() -> Tolk {
        unsafe { Tolk_Load(); }
        Tolk
    }
}

impl Drop for Tolk {
    fn drop(&mut self) {
        unsafe { Tolk_Unload(); }
    }
}

#[test]
fn test_drop_behavior() {
    unsafe {
        let f = Tolk::new();
        assert_eq!(Tolk_IsLoaded(), true);
    }

    unsafe { assert_eq!(Tolk_IsLoaded(), false); }
}