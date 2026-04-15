use crate::sdio_constants::*;

// Initialize, Read, and Write Utilities for an SD Card

pub fn sd_card_initialize(sd_card_id : SDCardId) {
    // Check that we are running with a SDHC version 1.0 and a SD Host Spec V3 as those are the only supported ones
    let cntrlr_vers: u16 = sdio_get_cntrlr_vers(sd_card_id);
    if cntrlr_vers != 0x1002 {
        // TODO Error?
        return
    }
    // The following are the steps from the TRM

    // Setup the SD Card Controller on the MPSoC
    // TODO: Are we sure we should do this everytime we boot? Should there be cases we do not do this for warm boots?
    //       Is there ever a case we want to do a partial reset? What is the state of the peripheral when we come out of each reset type?
    //       Can we ever come up with the sd card in an inconsistent state?
    sdio_software_reset_all(sd_card_id);
    // TODO timeout
    // TODO: How long does this take? Should we go do something else while this is resetting and then comeback? Is it even worth it?
    while sdio_all_in_reset(sd_card_id) {}
    let dev_capabilities : u64 = sdio_read_capabilities(sd_card_id); // only valid once not in reset (MPSoC Controllers are expected to have capabilities 0x280737EC6481, but unsure if there is a hardware configuration that can change this)
    sdio_set_power_cntrl_default(sd_card_id);
    sdio_set_clk_cntrl_default(sd_card_id, dev_capabilities); // This function may take awhile, we might be able to start something else while this finishes
    sdio_set_adma2_default(sd_card_id); // Why can't we use 64-bit ADMA2 mode from the start? Is it only supported once we know the type of SD card inserted?
    sdio_set_normal_interrupts_default(sd_card_id);
    sdio_set_error_interrupts_default(sd_card_id);
    sdio_set_normal_interrupts_sgs_default(sd_card_id);
    sdio_set_err_interrupts_sgs_default(sd_card_id);
    sdio_set_transfer_mode_default(sd_card_id);
    sdio_set_blocksize_default(sd_card_id);
    // Does the above match what is required in the specification? Physical Layer Simplified Specification Version 9.10 Section 4.2.1


    // Initialize the card (if inserted)
    // TODO: How do we want to do this? Will an interrupt be generated if we enable it for the insertion (this will play into the wait for insertion below)?
    //       Do we want to error if a card is not inserted? Or do we want to go into a mode where we wait for the card to be inserted?
    //       Do we want to go to a secondary boot device if the card is not inserted?
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
