//! PSP OS System API
//!
//! The names of functions and types beginning with `sce` or `Sce` were found by
//! reverse engineering various PSP games and OS versions.
//!
//! - `sceXYZ`: Sony API
//!     - `sceKernelXYZ`: Interface to the PSP OS kernel
//!     - `sceCtrlXYZ`: Button control API
//!     - `sceDisplayXYZ`: Display API
//!     - `sceGeXYZ`: Interface to the graphics chip (Graphics Engine)
//!     - `sceUsb`: USB API
//!         - `sceUsbCam`: USB camera
//!     - `scePower`: Power API
//!     - `sceWlan`: Wireless network API
//!     - `sceRtc`: Real time clock API
//!     - `sceIo`: File I/O API
//!     - `sceAudio`: Audio API
//!     - `sceAtrac`: Sony ATRAC3 Codec API
//!     - `sceJpeg`: JPEG decoding API
//!     - `sceUmd`: UMD Drive API
//!     - `sceMpeg`: MPEG codec API
//!     - `sceHprm`: Headphone Remote API (headphone accessory with controls)
//!     - `sceGu`: Graphics API (Similar to OpenGL)
//!     - `sceGum`: Matrix utility functions
//!     - `sceMp3`: MP3 decoder API
//!     - `sceRegistry`: PSP OS Registry API
//!     - `sceOpenPSID`: Console identification API (unique to every console)

#[macro_use]
mod macros;

mod ctrl;
pub use ctrl::*;

mod display;
pub use display::*;

mod ge;
pub use ge::*;

mod kernel;
pub use kernel::*;

mod usb;
pub use usb::*;

mod power;
pub use power::*;

mod wlan;
pub use wlan::*;

mod rtc;
pub use rtc::*;

mod io;
pub use io::*;

mod audio;
pub use audio::*;

mod atrac;
pub use atrac::*;

mod jpeg;
pub use jpeg::*;

mod umd;
pub use umd::*;

mod mpeg;
pub use mpeg::*;

mod hprm;
pub use hprm::*;

mod gu;
pub use gu::*;

mod gum;
pub use gum::*;

mod types;
pub use types::*;

mod mp3;
pub use mp3::*;

mod registry;
pub use registry::*;

mod openpsid;
pub use openpsid::*;

// These are not found (likely because this was tested in user mode on a PSP-2000).
// pub mod sircs;
// pub mod video;
// TODO: Add kernel module support to this crate.
// pub mod nand;

pub mod vfpu_context;

use core::ffi::c_void;

// http://uofw.github.io/uofw/structSceStubLibraryEntryTable.html
#[repr(C)]
pub struct SceStubLibraryEntry {
    pub name: *const u8,
    pub version: [u8; 2],
    pub flags: u16,
    pub len: u8,
    pub v_stub_count: u8,
    pub stub_count: u16,
    pub nid_table: *const u32,
    pub stub_table: *const c_void,
}

unsafe impl Sync for SceStubLibraryEntry {}

#[repr(C, packed)]
pub struct SceModuleInfo {
    pub mod_attribute: u16,
    pub mod_version: [u8; 2],
    pub mod_name: [u8; 27],
    pub terminal: u8,
    pub gp_value: *const u8,
    pub ent_top: *const u8,
    pub ent_end: *const u8,
    pub stub_top: *const u8,
    pub stub_end: *const u8,
}

unsafe impl Sync for SceModuleInfo {}

impl SceModuleInfo {
    #[doc(hidden)]
    pub const fn name(s: &str) -> [u8; 27] {
        let bytes = s.as_bytes();
        let mut result = [0; 27];

        let mut i = 0;
        while i < bytes.len() {
            result[i] = bytes[i];

            i += 1;
        }

        result
    }
}

#[repr(C, packed)]
pub struct SceLibraryEntry {
    pub name: *const u8,
    pub version: (u8, u8),
    pub attribute: SceLibAttr,
    pub entry_len: u8,
    pub var_count: u8,
    pub func_count: u16,
    pub entry_table: *const SceLibraryEntryTable,
}

unsafe impl Sync for SceLibraryEntry {}

bitflags::bitflags! {
    // https://github.com/uofw/uofw/blob/f099b78dc0937df4e7346e2e417b63f471f8a3af/include/loadcore.h#L152
    pub struct SceLibAttr: u16 {
        const SCE_LIB_NO_SPECIAL_ATTR = 0;
        const SCE_LIB_AUTO_EXPORT = 0x1;
        const SCE_LIB_WEAK_EXPORT = 0x2;
        const SCE_LIB_NOLINK_EXPORT = 0x4;
        const SCE_LIB_WEAK_IMPORT = 0x8;
        const SCE_LIB_SYSCALL_EXPORT = 0x4000;
        const SCE_LIB_IS_SYSLIB = 0x8000;
    }
}

pub struct SceLibraryEntryTable {
    pub module_start_nid: u32,
    pub module_info_nid: u32,
    pub module_start: unsafe extern "C" fn(isize, *const *const u8) -> isize,
    pub module_info: *const SceModuleInfo,
}

unsafe impl Sync for SceLibraryEntryTable {}
