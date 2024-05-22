#![no_std]

//!Crate for identifying the environment for Game Boy Advance ROMs.
//! The environment can be identified by simply calling the `get_env()` function. <br>
//! The supported environments can be found in the `Environment`-enum <br>

use core::arch::asm;
use core::ptr::{read_volatile, write_volatile};

const MEMCTRL_REGISTER: *mut u32 = 0x4000800 as *mut u32;
const EWRAM_STATIC_DATA: *mut i32 = 0x2000000 as *mut i32;

/// Represents the current system environment. <br>
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Environment {
    /// Nintendo DS fat or lite
    NintendoDS,
    /// MGBA or NanoBoyAdvance
    MGBA,
    /// No$GBA (debug mode)
    NoCashGBA,
    GameBoyAdvance,
    /// gpSP or MyBoy! Android Emulator
    GpSp,
    VisualBoyAdvance,
    GameBoyAdvanceMicro,
    Unknown,
}

/// Returns the current system environment. <br>
pub fn get_env() -> Environment {
    // the order of these checks is critical as the succeeding checks may crash the system
    match () {
        _ if identify_ds() => Environment::NintendoDS,
        _ if identify_mgba() => Environment::MGBA,
        _ if identify_nocashba_debug() => Environment::NoCashGBA,
        _ if identify_real_gba() => Environment::GameBoyAdvance,
        _ if identify_gba_micro() => Environment::GameBoyAdvanceMicro,
        _ if identify_gpsp() => Environment::GpSp,
        _ if identify_vba() => Environment::VisualBoyAdvance,
        _ => Environment::Unknown,
    }
}


#[inline(never)]
fn ram_test() -> bool {
    unsafe {
        write_volatile(EWRAM_STATIC_DATA, 0x70717518);
        let read_value = read_volatile(EWRAM_STATIC_DATA);
        write_volatile(EWRAM_STATIC_DATA, 0); // Clear the value to avoid false positives
        read_value == 0x70717518
    }
}

/// Should always return 0x0E000020 <br>
/// On NDS it will return an open bus value (i.e 0x6E156015) <br>
/// On a GBA Micro it will return 0x0D000020 <br>
#[inline(never)]
fn dram_training() -> u32 {
    let original_value = unsafe { read_volatile(MEMCTRL_REGISTER) };
    let base_value = original_value & !(0xF << 24); // Clear the bits we're going to modify
    let mut last_known_good_value = base_value;

    for i in 0..=0xE {
        let memctrl_value = base_value | (i << 24);
        unsafe {
            write_volatile(MEMCTRL_REGISTER, memctrl_value);
            if ram_test() {
                last_known_good_value = memctrl_value;
            } else {
                write_volatile(MEMCTRL_REGISTER, original_value); // Restore the original value
                return last_known_good_value;
            }
        }
    }

    // Restore the original value before returning
    unsafe {
        write_volatile(MEMCTRL_REGISTER, original_value);
    }

    last_known_good_value
}

/// Detects if the current system is a GBA Micro. <br>
/// DS: `false` <br>
/// mGBA: `false` <br>
/// No$GBA (debug): `false` <br>
/// No$GBA: `false` <br>
/// GBA: `true` <br>
/// GBA Micro: `true` <br>
/// MyBoy: not tested <br>
/// gpSP: `false` <br>
/// VBA: `false` <br>
pub fn identify_gba_micro() -> bool {
    dram_training() == 0x0D000020
}

/// Detects if the current system is a Nintendo DS running in GBA mode. <br>
/// DS: `false` <br>
/// mGBA: `false` <br>
/// No$GBA (debug): `false` <br>
/// No$GBA: `false` <br>
/// GBA: `false` <br>
/// MyBoy: not tested <br>
/// gpSP: `false` <br>
/// VBA: `false` <br>
#[inline(never)]
pub fn identify_ds() -> bool {
    let mut result: u32;
    unsafe {
        asm!(
            "svc #0x0D",            // System function 13
            "ldr r3, =0x4551E780",  // Load constant into r3
            "adds r0, r0, r3",      // Add r3 to r0, store in r0
            "rsbs r3, r0, #0",      // Reverse subtract 0 from r0, store in r3
            "adcs r0, r0, r3",      // Add r0 to r3 with carry, store in r0
            out("r0") result,       // Output the result from r0 to a Rust variable
            lateout("r3") _,        // Declare r3 as a clobbered register
            options(nostack, nomem) // This assembly does not affect stack or memory
        );
    }
    result != 0 // Compare the result to determine if the system is a DS
}

/// Detects if the system is running mGBA. <br>
/// DS: `false` <br>
/// mGBA: `true` <br>
/// No$GBA (debug): `false` <br>
/// No$GBA: `false` <br>
/// GBA: `false` <br>
/// MyBoy: not tested <br>
/// gpSP: `false` <br>
/// VBA: `false` <br>
#[inline(never)]
pub fn identify_mgba() -> bool {
    const REG_MGBA_ENABLE: *mut u16 = 0x04FFF780 as *mut u16;
    let original_value = unsafe { read_volatile(REG_MGBA_ENABLE) };

    unsafe {
        write_volatile(REG_MGBA_ENABLE, 0xC0DE);
        let result = read_volatile(REG_MGBA_ENABLE) == 0x1DEA;
        write_volatile(REG_MGBA_ENABLE, original_value); // Restore original value
        result
    }
}

/// Detects if the system is running no$gba debug. <br>
/// DS: `false` <br>
/// mGBA: `false` <br>
/// No$GBA (debug): `true` <br>
/// No$GBA: `false` <br>
/// GBA: `false` <br>
/// MyBoy: not tested <br>
/// gpSP: `false` <br>
/// VBA: `false` <br>
#[inline(never)]
pub fn identify_nocashba_debug() -> bool {
    const NOCASH_SIG: *const [u8; 7] = 0x04FFFA00 as *const [u8; 7];
    const NOCASH_SIG_STR: &[u8; 7] = b"no$gba ";

    unsafe { read_volatile(NOCASH_SIG) == *NOCASH_SIG_STR }
}

/// Detects if the system is a real Game Boy Advance. <br>
/// DS: `false` <br>
/// mGBA: `true` <br>
/// No$GBA (debug): `true` <br>
/// No$GBA: `true` <br>
/// GBA: `true` <br>
/// MyBoy: not tested <br>
/// gpSP: `false` <br>
/// VBA: `false` <br>
#[inline(never)]
pub fn identify_real_gba() -> bool {
    unsafe {
        let memctrl_reg = read_volatile(MEMCTRL_REGISTER);
        memctrl_reg == 0x0D000020 || memctrl_reg == 0x0E000020
    }
}

fn ram_overclock() -> bool {
    unsafe {
        write_volatile(MEMCTRL_REGISTER, 0x0E000020);
        static mut EWRAM_STATIC_DATA: i32 = 0;
        let ewram_static_data = &mut EWRAM_STATIC_DATA as *mut i32;

        write_volatile(ewram_static_data, 1);

        if read_volatile(ewram_static_data) != 1 {
            write_volatile(MEMCTRL_REGISTER, 0x0D000020);
            false
        } else {
            true
        }
    }
}

/// Detects if the system is running the MyBoy emulator. <br>
/// DS: `false` <br>
/// mGBA: `false` <br>
/// No$GBA (debug): `false` <br>
/// No$GBA: `false` <br>
/// GBA: `false` / crash (not sure yet) <br>
/// MyBoy: not tested <br>
/// gpSP: `true` <br>
/// VBA: `false` <br>
#[inline(never)]
pub fn identify_gpsp() -> bool {
    const MODE_0: u16 = 0;
    const BG0_ENABLE: u16 = 1 << 8;
    const REG_DISPCNT: *mut u16 = 0x4000000 as *mut u16;

    unsafe {
        let prev_dispcnt: u16 = read_volatile(REG_DISPCNT);
        write_volatile(REG_DISPCNT, MODE_0 | BG0_ENABLE);

        ram_overclock();

        let identified: bool = (read_volatile(REG_DISPCNT) & BG0_ENABLE) == 0;

        write_volatile(REG_DISPCNT, prev_dispcnt);

        identified
    }
}

/// Detects if the system is running VisualBoyAdvance. <br>
/// DS: crash <br>
/// mGBA: `true` <br>
/// No$GBA (debug): crash <br>
/// No$GBA: crash <br>
/// GBA: crash <br>
/// MyBoy: not tested <br>
/// gpSP: crash <br>
/// VBA: `true` <br>
#[inline(never)]
pub fn identify_vba() -> bool {
    const TEST_MESSAGE: &str = "VBA";
    let identified: bool;
    unsafe {
        asm!(
        "movs r0, {0}",
        "svc  #255",
        in(reg) TEST_MESSAGE.as_ptr(),
        );
        asm!(
        "movs r0, {0}",
        "svc  #255",
        in(reg) "\n".as_ptr(),
        );
        identified = true;
    }
    identified
}