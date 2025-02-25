//! Hardware related commands for Neotron OS

use crate::{bios, println, Ctx, API};

pub static LSHW_ITEM: menu::Item<Ctx> = menu::Item {
    item_type: menu::ItemType::Callback {
        function: lshw,
        parameters: &[],
    },
    command: "lshw",
    help: Some("List all the BIOS hardware"),
};

/// Called when the "lshw" command is executed.
fn lshw(_menu: &menu::Menu<Ctx>, _item: &menu::Item<Ctx>, _args: &[&str], _ctx: &mut Ctx) {
    let api = API.get();
    let mut found = false;

    println!("Memory regions:");
    for region_idx in 0..=255u8 {
        if let bios::Option::Some(region) = (api.memory_get_region)(region_idx) {
            println!("  {}: {}", region_idx, region);
            found = true;
        }
    }
    if !found {
        println!("  None");
    }

    found = false;

    println!("Serial Devices:");
    for dev_idx in 0..=255u8 {
        if let bios::Option::Some(device_info) = (api.serial_get_info)(dev_idx) {
            println!(
                "  {}: {} {:?}",
                dev_idx, device_info.name, device_info.device_type
            );
            found = true;
        }
    }
    if !found {
        println!("  None");
    }

    found = false;

    println!("Block Devices:");
    for dev_idx in 0..=255u8 {
        if let bios::Option::Some(device_info) = (api.block_dev_get_info)(dev_idx) {
            println!(
                "  {}: {} {:?} bs={} size={} MiB",
                dev_idx,
                device_info.name,
                device_info.device_type,
                device_info.block_size,
                (device_info.num_blocks * u64::from(device_info.block_size)) / (1024 * 1024)
            );
            found = true;
        }
    }
    if !found {
        println!("  None");
    }

    found = false;

    println!("I2C Buses:");
    for dev_idx in 0..=255u8 {
        if let bios::Option::Some(device_info) = (api.i2c_bus_get_info)(dev_idx) {
            println!("  {}: {:?}", dev_idx, device_info);
            found = true;
        }
    }
    if !found {
        println!("  None");
    }

    found = false;

    println!("Neotron Bus Devices:");
    for dev_idx in 0..=255u8 {
        if let bios::Option::Some(device_info) = (api.bus_get_info)(dev_idx) {
            println!("  {}: {:?}", dev_idx, device_info);
            found = true;
        }
    }
    if !found {
        println!("  None");
    }

    found = false;

    println!("Audio Mixers:");
    for dev_idx in 0..=255u8 {
        if let bios::Option::Some(device_info) = (api.audio_mixer_channel_get_info)(dev_idx) {
            println!("  {}: {:?}", dev_idx, device_info);
            found = true;
        }
    }
    if !found {
        println!("  None");
    }
}
