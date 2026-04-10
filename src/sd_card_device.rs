// Initialize, Read, and Write Utilities for an SD Card

pub fn sd_card_initialize(address : u64) {
    //TODO What needs to be done? Is it different between warm and cold boots?
}

pub fn sd_card_sync_read(address : u64, bytes_to_read: u64, buffer:&mut [u8]) -> () {
    //TODO How do we talk to the SD card from an MPSoC?
}

pub fn sd_card_async_read(address : u64, bytes_to_read: u64, buffer:&mut [u8]) -> () {
    //TODO How do we talk to the SD card from an MPSoC?
}

pub fn sd_card_async_read_complete() -> bool {
    //TODO How do we talk to the SD card from an MPSoC?
}

pub fn sd_card_sync_write(address : u64, bytes_to_read: u64, buffer:&mut [u8]) -> () {
    //TODO How do we talk to the SD card from an MPSoC?
}

pub fn sd_card_async_write(address : u64, bytes_to_read: u64, buffer:&mut [u8]) -> () {
    //TODO How do we talk to the SD card from an MPSoC?
}

pub fn sd_card_async_write_complete() -> bool {
    //TODO How do we talk to the SD card from an MPSoC?
}
