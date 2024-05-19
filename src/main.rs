#![no_std]
#![no_main]

use agb::println;
use core::arch::asm;
use agb::{
    display::{
        tiled::{RegularBackgroundSize, TileFormat, TiledMap},
        Font, Priority,
    },
    include_font,
};

use core::fmt::Write;

use core::ptr::{read_volatile, write_volatile};

static FONT: Font = include_font!("fnt/ark-pixel-10px-proportional-ja.ttf", 10);

static CHAR_WIDTHS: [u8; 256] = [
    // Control characters (0-31)
    8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,
    // Space to / (32-47)
    8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,
    // 0 to 9 (48-57)
    8,8,8,8,8,8,8,8,8,8,
    // : to @ (58-64)
    8,8,8,8,8,8,8,
    // A to Z (65-90)
    8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,
    // [ to ` (91-96)
    8,8,8,8,8,8,
    // a to z (97-122)
    8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,
    // { to DEL (123-127)
    8,8,8,8,8,
    // Extended ASCII (128-255)
    8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,
    8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,
    8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,
    8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,
];

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let (gfx, mut vram) = gba.display.video.tiled0();
    let vblank = agb::interrupt::VBlank::get();

    vram.set_background_palette_raw(&[
        0x0000, 0x0ff0, 0x0fff, 0xf00f, 0xf0f0, 0x0f0f, 0xaaaa, 0x5555, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
    ]);

    let background_tile = vram.new_dynamic_tile().fill_with(0);

    let mut bg = gfx.background(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    for y in 0..20u16 {
        for x in 0..30u16 {
            bg.set_tile(
                &mut vram,
                (x, y),
                &background_tile.tile_set(),
                background_tile.tile_setting(),
            );
        }
    }

    vram.remove_dynamic_tile(background_tile);

    let mut renderer = FONT.render_text((0u16, 0u16));
    let mut writer = renderer.writer(1, 0, &mut bg, &mut vram);

    let ds_detected = false;
    let mgba_detected = false;
    let gba_detected = false;
    let myboy_detected = false;

    // Display the results using your rendering system
    // agb::println!("DS detected: {}", ds_detected);
    // agb::println!("mGBA detected: {}", mgba_detected);
    // agb::println!("GBA detected: {}", gba_detected);
    // agb::println!("MyBoy detected: {}", myboy_detected);

    //26 characters
    writeln!(&mut writer, "tolik518@gba").unwrap();
    writeln!(&mut writer, "------------").unwrap();

    writer = renderer.writer(2, 0, &mut bg, &mut vram);
    write!(&mut writer, "System: ").unwrap();
    writer = renderer.writer(1, 0, &mut bg, &mut vram);
    if ds_detected {
        writeln!(&mut writer, "Nintendo DS").unwrap();
    } else if mgba_detected {
        writeln!(&mut writer, "mGBA").unwrap();
    } else if gba_detected {
        writeln!(&mut writer, "GameBoyAdvance").unwrap();
    } else if myboy_detected {
        writeln!(&mut writer, "MyBoy").unwrap();
    } else {
        writeln!(&mut writer, "Unknown").unwrap();
    }

    writer = renderer.writer(2, 0, &mut bg, &mut vram);
    write!(&mut writer, "Uptime: ").unwrap();
    writer = renderer.writer(1, 0, &mut bg, &mut vram);
    writeln!(&mut writer, "0:00:00").unwrap();

    writer = renderer.writer(2, 0, &mut bg, &mut vram);
    write!(&mut writer, "Resolution: ").unwrap();
    writer = renderer.writer(1, 0, &mut bg, &mut vram);
    writeln!(&mut writer, "240x160px").unwrap();

    writer = renderer.writer(2, 0, &mut bg, &mut vram);
    write!(&mut writer, "CPU: ").unwrap();
    writer = renderer.writer(1, 0, &mut bg, &mut vram);
    writeln!(&mut writer, "ARM7TDMI @ 16.78 MHz").unwrap();

    writer = renderer.writer(2, 0, &mut bg, &mut vram);
    write!(&mut writer, "RAM: ").unwrap();
    writer = renderer.writer(1, 0, &mut bg, &mut vram);
    // show here free memory
    writeln!(&mut writer, "256 KB").unwrap();

    writer = renderer.writer(2, 0, &mut bg, &mut vram);
    write!(&mut writer, "VRAM: ").unwrap();
    writer = renderer.writer(1, 0, &mut bg, &mut vram);
    writeln!(&mut writer, "96 KB").unwrap();

    writer = renderer.writer(2, 0, &mut bg, &mut vram);
    write!(&mut writer, "eRAM: ").unwrap();
    writer = renderer.writer(1, 0, &mut bg, &mut vram);
    writeln!(&mut writer, "32 KB").unwrap();

    writer.commit();

    bg.commit(&mut vram);
    bg.set_visible(true);

    let mut frame = 0;

    loop {
        frame += 1;
        // get free memory

        vblank.wait_for_vblank();
        bg.commit(&mut vram);

        renderer.clear(&mut vram);
    }
}

#[inline(never)]
fn detect_ds() -> bool {
    // DS detection via BIOS checksum syscall
    let checksum: u32;
    unsafe {
        asm!(
        "swi 0x04", // BIOS Checksum SWI number
        out("r0") checksum,
        options(nostack, nomem),
        );
    }
    checksum == 0x0F56F
}

#[inline(never)]
fn detect_mgba() -> bool {
    // mGBA detection via debug registers
    let debug_reg: u32;
    unsafe {
        debug_reg = read_volatile(0x4FFF780 as *const u32);
    }
    debug_reg == 0xDEADBEEF
}

#[inline(never)]
fn detect_gba_memory_control_register() -> bool {
    // GBA detection via memory control register presence
    let memctrl_reg: u32;
    unsafe {
        memctrl_reg = read_volatile(0x4000800 as *const u32);
    }
    memctrl_reg == 0x0D000020 || memctrl_reg == 0x0E000020
}

#[inline(never)]
fn ram_overclock() -> bool {
    // volatile unsigned& memctrl_register = *reinterpret_cast<unsigned*>(0x4000800);
    const MEMCTRL_REGISTER: *mut u32 = 0x4000800 as *mut u32;
    const EWRAM_STATIC_DATA: *mut i32 = 0x2000000 as *mut i32;

    unsafe {
        // memctrl_register = 0x0E000020;
        write_volatile(MEMCTRL_REGISTER, 0x0E000020);

        // volatile int& ewram_static_data = _ewram_static_data;
        // ewram_static_data = 1;
        write_volatile(EWRAM_STATIC_DATA, 1);

        // if (ewram_static_data != 1) {
        if read_volatile(EWRAM_STATIC_DATA) != 1 {
            // memctrl_register = 0x0D000020;
            write_volatile(MEMCTRL_REGISTER, 0x0D000020);
            return false;
        } else {
            return true;
        }
    }
}

#[inline(never)]
fn detect_android_myboy_emulator() -> bool {
    const MODE_0: u16 = 0;
    const BG0_ENABLE: u16 = 1 << 8;
    const REG_DISPCNT: *mut u16 = 0x4000000 as *mut u16;

    unsafe {
        // const u16 prev_dispcnt = REG_DISPCNT;
        let prev_dispcnt: u16 = read_volatile(REG_DISPCNT);

        // REG_DISPCNT = MODE_0 | BG0_ENABLE;
        write_volatile(REG_DISPCNT, MODE_0 | BG0_ENABLE);

        // RAM overclocking in MyBoy! erroneously clears REG_DISPCNT.
        ram_overclock();

        // const bool detected = not (REG_DISPCNT & BG0_ENABLE);
        let detected: bool = (read_volatile(REG_DISPCNT) & BG0_ENABLE) == 0;

        // REG_DISPCNT = prev_dispcnt;
        write_volatile(REG_DISPCNT, prev_dispcnt);

        return detected;
    }
}