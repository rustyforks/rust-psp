#![no_std]
#![no_main]

use psp::sys;

psp::module!("sample_module", 1, 1);

const BUFFER_WIDTH: usize = 512;
const DISPLAY_HEIGHT: usize = 272;
const DISPLAY_WIDTH: usize = 480;
static mut VRAM: *mut u32 = 0x4000_0000 as *mut u32;

fn psp_main() {
    psp::enable_home_button();
    unsafe {
        sys::sceDisplaySetMode(sys::DisplayMode::Lcd, DISPLAY_WIDTH, DISPLAY_HEIGHT);

        // Cache-through address
        VRAM = (0x4000_0000u32 | sys::sceGeEdramGetAddr() as u32) as *mut u32;

        sys::sceDisplaySetFrameBuf(
            VRAM as *const u8,
            BUFFER_WIDTH,
            sys::DisplayPixelFormat::Psm8888,
            sys::DisplaySetBufSync::NextFrame,
        );

        loop {
            sys::sceDisplayWaitVblankStart();
            for pos in 0..255  {
                let color = wheel(pos);

                for i in 0..(BUFFER_WIDTH * DISPLAY_HEIGHT) {
                    *VRAM.add(i) = color;
                }
            }
        }
    }
}

fn wheel(mut pos: u8) -> u32 {
    pos = 255 - pos;
    if pos < 85 {
        u32::from_be_bytes([255 - pos * 3, 0, pos * 3, 255])
    } else if pos < 170 {
        pos -= 85;
        u32::from_be_bytes([0, pos * 3, 255 - pos * 3, 255])
    } else {
        pos -= 170;
        u32::from_be_bytes([pos * 3, 255 - pos * 3, 0, 255])
    }
}
