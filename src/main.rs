#![no_std]
#![no_main]

extern crate alloc;

mod gba_env;

use alloc::borrow::ToOwned;
use alloc::string::ToString;
use core::arch::asm;
use agb::{
    display::{
        tiled::{RegularBackgroundSize, TileFormat, TiledMap},
        Font, Priority,
    },
    include_font,
};

use core::fmt::{Debug, Write};

use core::ptr::{read_volatile, write_volatile};
static FONT: Font = include_font!("fnt/ark-pixel-10px-proportional-ja.ttf", 10);

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let (gfx, mut vram) = gba.display.video.tiled0();
    let vblank = agb::interrupt::VBlank::get();

    vram.set_background_palette_raw(&[
        0x000B, 0x0ff0, 0x0fff, 0xf00f, 0xf0f0, 0x0f0f, 0xaaaa, 0x5555, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
    ]);
    let _ = vram.new_dynamic_tile().fill_with(0);

    let mut bg = gfx.background(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    let mut renderer = FONT.render_text((0u16, 0u16));
    let mut input = agb::input::ButtonController::new();
    let mut writer = renderer.writer(1, 0, &mut bg, &mut vram);

    writeln!(writer, "System: {:?}", gba_env::get_env()).unwrap();
    writeln!(writer, "Press any button...").unwrap();
    writer.commit();

    loop {
        vblank.wait_for_vblank();
        input.update();

        let mut writer = renderer.writer(1, 0, &mut bg, &mut vram);
        if input.is_pressed(agb::input::Button::from_bits_retain(0b11_1111_1111)) {
            writeln!(writer, "Any button was pressed, thanks for tuning in.").unwrap();
        }
        writer.commit();

        bg.commit(&mut vram);
        bg.set_visible(true);

        renderer.clear(&mut vram);
    }
}