//! Screen-related commands for Neotron OS

use neotron_common_bios::video::{Attr, TextBackgroundColour, TextForegroundColour};

use crate::{println, Ctx, API, VGA_CONSOLE};

pub static CLEAR_ITEM: menu::Item<Ctx> = menu::Item {
    item_type: menu::ItemType::Callback {
        function: clear,
        parameters: &[],
    },
    command: "screen_clear",
    help: Some("Clear the screen"),
};

pub static FILL_ITEM: menu::Item<Ctx> = menu::Item {
    item_type: menu::ItemType::Callback {
        function: fill,
        parameters: &[],
    },
    command: "screen_fill",
    help: Some("Fill the screen with characters"),
};

/// Called when the "clear" command is executed.
fn clear(_menu: &menu::Menu<Ctx>, _item: &menu::Item<Ctx>, _args: &[&str], _ctx: &mut Ctx) {
    if let Some(ref mut console) = unsafe { &mut VGA_CONSOLE } {
        console.clear();
    }
}

/// Called when the "fill" command is executed.
fn fill(_menu: &menu::Menu<Ctx>, _item: &menu::Item<Ctx>, _args: &[&str], _ctx: &mut Ctx) {
    if let Some(ref mut console) = unsafe { &mut VGA_CONSOLE } {
        console.clear();
        let api = API.get();
        let mode = (api.video_get_mode)();
        let (Some(width), Some(height)) = (mode.text_width(), mode.text_height()) else {
            println!("Unable to get console size");
            return;
        };
        // A range of printable ASCII compatible characters
        let mut char_cycle = (b' '..=b'~').cycle();
        let mut remaining = height * width;

        // Scroll two screen fulls
        'outer: for bg in (0..=7).cycle() {
            let bg_colour = TextBackgroundColour::new(bg).unwrap();
            for fg in 1..=15 {
                if fg == bg {
                    continue;
                }
                let fg_colour = TextForegroundColour::new(fg).unwrap();
                remaining -= 1;
                if remaining == 0 {
                    break 'outer;
                }
                let attr = Attr::new(fg_colour, bg_colour, false);
                let glyph = char_cycle.next().unwrap();
                console.set_attr(attr);
                console.write_bstr(&[glyph]);
            }
        }
        let attr = Attr::new(
            TextForegroundColour::WHITE,
            TextBackgroundColour::BLACK,
            false,
        );
        console.set_attr(attr);
    }
}
