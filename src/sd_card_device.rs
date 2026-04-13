use crate::sdio_constants::*;

// Initialize, Read, and Write Utilities for an SD Card

pub fn sd_card_initialize(sd_card_id : SDCardId) {
    // The following are the steps from the TRM


    // Setup the SD Card Controller on the MPSoC
    // TODO: Are we sure we should do this everytime we boot? Should there be cases we do not do this for warm boots?
    //       Is there ever a case we want to do a partial reset?
    sdio_software_reset_all(sd_card_id);
    // TODO timeout
    // TODO: How long does this take? Should we go do something else while this is resetting and then comeback?
    while !sdio_all_in_reset(sd_card_id) {}



    // Initialize the card (if inserted)
}

/*
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

pub fn sd_check_fat_partition() {
    // TODO
}
    */
