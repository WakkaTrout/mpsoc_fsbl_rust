// SDIO Configuration Registers

// All Values below are from UG 1087 (https://docs.amd.com/r/en-US/ug1087-zynq-ultrascale-registers)

//SD0 Config Register Baseaddress
const SD0_CONF_REG_BASEADDRESS                             : u32 = 0xFF160000;

//SD1 Config Register Baseaddress
const SD1_CONF_REG_BASEADDRESS                             : u32 = 0xFF170000;

// SDIO Register Offsets
const SDIO_REG_SDMASYSADDRLO_REG_OFFSET                    : u32 = 0x00000000;
const SDIO_REG_SDMASYSADDRHI_REG_OFFSET                    : u32 = 0x00000002;
const SDIO_REG_BLOCKSIZE_REG_OFFSET                        : u32 = 0x00000004;
const SDIO_REG_BLOCKCOUNT_REG_OFFSET                       : u32 = 0x00000006;
const SDIO_REG_ARGUMENT1LO_REG_OFFSET                      : u32 = 0x00000008;
const SDIO_REG_ARGUMENT1HI_REG_OFFSET                      : u32 = 0x0000000A;
const SDIO_REG_TRANSFERMODE_REG_OFFSET                     : u32 = 0x0000000C;
const SDIO_REG_COMMAND_REG_OFFSET                          : u32 = 0x0000000E;
const SDIO_REG_RESPONSE0_REG_OFFSET                        : u32 = 0x00000010;
const SDIO_REG_RESPONSE1_REG_OFFSET                        : u32 = 0x00000012;
const SDIO_REG_RESPONSE2_REG_OFFSET                        : u32 = 0x00000014;
const SDIO_REG_RESPONSE3_REG_OFFSET                        : u32 = 0x00000016;
const SDIO_REG_RESPONSE4_REG_OFFSET                        : u32 = 0x00000018;
const SDIO_REG_RESPONSE5_REG_OFFSET                        : u32 = 0x0000001A;
const SDIO_REG_RESPONSE6_REG_OFFSET                        : u32 = 0x0000001C;
const SDIO_REG_RESPONSE7_REG_OFFSET                        : u32 = 0x0000001E;
const SDIO_REG_DATAPORT_REG_OFFSET                         : u32 = 0x00000020;
const SDIO_REG_PRESENTSTATE_REG_OFFSET                     : u32 = 0x00000024;
const SDIO_REG_HOSTCONTROL1_REG_OFFSET                     : u32 = 0x00000028;
const SDIO_REG_POWERCONTROL_REG_OFFSET                     : u32 = 0x00000029;
const SDIO_REG_BLOCKGAPCONTROL_REG_OFFSET                  : u32 = 0x0000002A;
const SDIO_REG_WAKEUPCONTROL_REG_OFFSET                    : u32 = 0x0000002B;
const SDIO_REG_CLOCKCONTROL_REG_OFFSET                     : u32 = 0x0000002C;
const SDIO_REG_TIMEOUTCONTROL_REG_OFFSET                   : u32 = 0x0000002E;
const SDIO_REG_SOFTWARERESET_REG_OFFSET                    : u32 = 0x0000002F;
const SDIO_REG_NORMALINTRSTS_REG_OFFSET                    : u32 = 0x00000030;
const SDIO_REG_ERRORINTRSTS_REG_OFFSET                     : u32 = 0x00000032;
const SDIO_REG_NORMALINTRSTSENA_REG_OFFSET                 : u32 = 0x00000034;
const SDIO_REG_ERRORINTRSTSENA_REG_OFFSET                  : u32 = 0x00000036;
const SDIO_REG_NORMALINTRSIGENA_REG_OFFSET                 : u32 = 0x00000038;
const SDIO_REG_ERRORINTRSIGENA_REG_OFFSET                  : u32 = 0x0000003A;
const SDIO_REG_AUTOCMDERRSTS_REG_OFFSET                    : u32 = 0x0000003C;
const SDIO_REG_HOSTCONTROL2_REG_OFFSET                     : u32 = 0x0000003E;
const SDIO_REG_CAPABILITIES_REG_OFFSET                     : u32 = 0x00000040;
const SDIO_REG_MAXCURRENTCAP_REG_OFFSET                    : u32 = 0x00000048;
const SDIO_REG_FORCEEVENTFORAUTOCMDERRORSTATUS_REG_OFFSET  : u32 = 0x00000050;
const SDIO_REG_FORCEEVENTFORERRINTSTS_REG_OFFSET           : u32 = 0x00000052;
const SDIO_REG_ADMAERRSTS_REG_OFFSET                       : u32 = 0x00000054;
const SDIO_REG_ADMASYSADDR0_REG_OFFSET                     : u32 = 0x00000058;
const SDIO_REG_ADMASYSADDR1_REG_OFFSET                     : u32 = 0x0000005A;
const SDIO_REG_ADMASYSADDR2_REG_OFFSET                     : u32 = 0x0000005C;
const SDIO_REG_ADMASYSADDR3_REG_OFFSET                     : u32 = 0x0000005E;
const SDIO_REG_PRESETVALUE0_REG_OFFSET                     : u32 = 0x00000060;
const SDIO_REG_PRESETVALUE1_REG_OFFSET                     : u32 = 0x00000062;
const SDIO_REG_PRESETVALUE2_REG_OFFSET                     : u32 = 0x00000064;
const SDIO_REG_PRESETVALUE3_REG_OFFSET                     : u32 = 0x00000066;
const SDIO_REG_PRESETVALUE4_REG_OFFSET                     : u32 = 0x00000068;
const SDIO_REG_PRESETVALUE5_REG_OFFSET                     : u32 = 0x0000006A;
const SDIO_REG_PRESETVALUE6_REG_OFFSET                     : u32 = 0x0000006C;
const SDIO_REG_PRESETVALUE7_REG_OFFSET                     : u32 = 0x0000006E;
const SDIO_REG_BOOTTIMEOUTCNT_REG_OFFSET                   : u32 = 0x00000070;
const SDIO_REG_SLOTINTRSTS_REG_OFFSET                      : u32 = 0x000000FC;
const SDIO_REG_HOSTCONTROLLERVER_REG_OFFSET                : u32 = 0x000000FE;

// Register constants

const SDIO_REG_POWERCONTROL_EMMC_HWRESET_MASK              : u8 = 0x10;
const SDIO_REG_POWERCONTROL_EMMC_HWRESET_OFF               : u8 = 4;
const SDIO_REG_POWERCONTROL_PWCTRL_SDBUSVOLTAGE_MASK       : u8 = 0xE;
const SDIO_REG_POWERCONTROL_PWCTRL_SDBUSVOLTAGE_OFF        : u8 = 1;
const SDIO_REG_POWERCONTROL_PWCTRL_SDBUSPOWER_MASK         : u8 = 0x1;
const SDIO_REG_POWERCONTROL_PWCTRL_SDBUSPOWER_OFF          : u8 = 0;

const POWERCONTROL_VOLTAGE_1V8                             : u8 = 6;
const POWERCONTROL_VOLTAGE_3V0                             : u8 = 7;
const POWERCONTROL_VOLTAGE_3V3                             : u8 = 8;

const POWERCONTROL_POWER_OFF                               : u8 = 0;
const POWERCONTROL_POWER_ON                                : u8 = 1;

const SDIO_REG_CLOCKCONTROL_CLKCTRL_SDCLKFREQSEL_MASK      : u16 = 0xFF00;
const SDIO_REG_CLOCKCONTROL_CLKCTRL_SDCLKFREQSEL_OFF       : u16 = 8;
const SDIO_REG_CLOCKCONTROL_CLKCTRL_SDCLKFREQSEL_UPPERBITS_MASK  : u16 = 0x00C0;
const SDIO_REG_CLOCKCONTROL_CLKCTRL_SDCLKFREQSEL_UPPERBITS_OFF   : u16 = 6;
const SDIO_REG_CLOCKCONTROL_CLKCTRL_CLKGENSEL_MASK         : u16 = 0x0020;
const SDIO_REG_CLOCKCONTROL_CLKCTRL_CLKGENSEL_OFF          : u16 = 5;
const SDIO_REG_CLOCKCONTROL_CLKCTRL_SDCLKENA_MASK          : u16 = 0x0004;
const SDIO_REG_CLOCKCONTROL_CLKCTRL_SDCLKENA_OFF           : u16 = 2;
const SDIO_REG_CLOCKCONTROL_SDHCCLKGEN_INTCLKSTABLE_DSYNC_MASK   : u16 = 0x0002;
const SDIO_REG_CLOCKCONTROL_SDHCCLKGEN_INTCLKSTABLE_DSYNC_OFF    : u16 = 1;
const SDIO_REG_CLOCKCONTROL_CLKCTRL_INTCLKENA_MASK         : u16 = 0x0001;
const SDIO_REG_CLOCKCONTROL_CLKCTRL_INTCLKENA_OFF          : u16 = 0;

const CLOCKCONTROL_SD_CLK_DISABLE                          : u16 = 0;
const CLOCKCONTROL_SD_CLK_ENABLE                           : u16 = 1;

const CLOCKCONTROL_HC_CLK_DISABLE                          : u16 = 0;
const CLOCKCONTROL_HC_CLK_ENABLE                           : u16 = 1;

const SDIO_REG_SOFTWARERESET_SWRESET_FOR_DAT_MASK          : u8 = 0x4;
const SDIO_REG_SOFTWARERESET_SWRESET_FOR_DAT_OFF           : u8 = 2;
const SDIO_REG_SOFTWARERESET_SWRESET_FOR_CMD_MASK          : u8 = 0x2;
const SDIO_REG_SOFTWARERESET_SWRESET_FOR_CMD_OFF           : u8 = 1;
const SDIO_REG_SOFTWARERESET_SWRESET_FOR_ALL_MASK          : u8 = 0x1;
const SDIO_REG_SOFTWARERESET_SWRESET_FOR_ALL_OFF           : u8 = 0;

#[repr(u8)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub enum SDCardId {
    SD0 = 0,
    SD1 = 1,
}

#[inline(always)]
pub unsafe fn sdio_reg_write_u32(sd_card_id : SDCardId, reg: u32, val: u32) -> () {
    let sdio_reg : *mut u32 = (SD0_CONF_REG_BASEADDRESS + ( (sd_card_id as u32) << 16 ) + reg) as *mut u32;
    unsafe {
        sdio_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn sdio0_reg_write_u32(reg: u32, val: u32) -> () {
    let sdio_reg : *mut u32 = (SD0_CONF_REG_BASEADDRESS + reg) as *mut u32;
    unsafe {
        sdio_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn sdio1_reg_write_u32(reg: u32, val: u32) -> () {
    let sdio_reg : *mut u32 = (SD1_CONF_REG_BASEADDRESS + reg) as *mut u32;
    unsafe {
        sdio_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn sdio_reg_write_u16(sd_card_id : SDCardId, reg: u32, val: u16) -> () {
    let sdio_reg : *mut u16 = (SD0_CONF_REG_BASEADDRESS + ( (sd_card_id as u32) << 16 ) + reg) as *mut u16;
    unsafe {
        sdio_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn sdio0_reg_write_u16(reg: u32, val: u16) -> () {
    let sdio_reg : *mut u16 = (SD0_CONF_REG_BASEADDRESS + reg) as *mut u16;
    unsafe {
        sdio_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn sdio1_reg_write_u16(reg: u32, val: u16) -> () {
    let sdio_reg : *mut u16 = (SD1_CONF_REG_BASEADDRESS + reg) as *mut u16;
    unsafe {
        sdio_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn sdio_reg_write_u8(sd_card_id : SDCardId, reg: u32, val: u8) -> () {
    let sdio_reg : *mut u8 = (SD0_CONF_REG_BASEADDRESS + ( (sd_card_id as u32) << 16 ) + reg) as *mut u8;
    unsafe {
        sdio_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn sdio0_reg_write_u8(reg: u32, val: u8) -> () {
    let sdio_reg : *mut u8 = (SD0_CONF_REG_BASEADDRESS + reg) as *mut u8;
    unsafe {
        sdio_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn sdio1_reg_write_u8(reg: u32, val: u8) -> () {
    let sdio_reg : *mut u8 = (SD1_CONF_REG_BASEADDRESS + reg) as *mut u8;
    unsafe {
        sdio_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn sdio_reg_read_u64(sd_card_id : SDCardId, reg: u32) -> u64 {
    let sdio_reg : *mut u64 = (SD0_CONF_REG_BASEADDRESS + ( (sd_card_id as u32) << 16 ) + reg) as *mut u64;
    unsafe {
        return sdio_reg.read_volatile();
    }
}

#[inline(always)]
pub unsafe fn sdio0_reg_read_u64(reg: u32) -> u64 {
    let sdio_reg : *mut u64 = (SD0_CONF_REG_BASEADDRESS + reg) as *mut u64;
    unsafe {
        return sdio_reg.read_volatile();
    }
}

#[inline(always)]
pub unsafe fn sdio1_reg_read_u64(reg: u32) -> u64 {
    let sdio_reg : *mut u64 = (SD1_CONF_REG_BASEADDRESS + reg) as *mut u64;
    unsafe {
        return sdio_reg.read_volatile();
    }
}

#[inline(always)]
pub unsafe fn sdio_reg_read_u16(sd_card_id : SDCardId, reg: u32) -> u16 {
    let sdio_reg : *mut u16 = (SD0_CONF_REG_BASEADDRESS + ( (sd_card_id as u32) << 16 ) + reg) as *mut u16;
    unsafe {
        return sdio_reg.read_volatile();
    }
}

#[inline(always)]
pub unsafe fn sdio0_reg_read_u16(reg: u32) -> u16 {
    let sdio_reg : *mut u16 = (SD0_CONF_REG_BASEADDRESS + reg) as *mut u16;
    unsafe {
        return sdio_reg.read_volatile();
    }
}

#[inline(always)]
pub unsafe fn sdio1_reg_read_u16(reg: u32) -> u16 {
    let sdio_reg : *mut u16 = (SD1_CONF_REG_BASEADDRESS + reg) as *mut u16;
    unsafe {
        return sdio_reg.read_volatile();
    }
}

#[inline(always)]
pub unsafe fn sdio_reg_read_u8(sd_card_id : SDCardId, reg: u32) -> u8 {
    let sdio_reg : *mut u8 = (SD0_CONF_REG_BASEADDRESS + ( (sd_card_id as u32) << 16 ) + reg) as *mut u8;
    unsafe {
        return sdio_reg.read_volatile();
    }
}

#[inline(always)]
pub unsafe fn sdio0_reg_read_u8(reg: u32) -> u8 {
    let sdio_reg : *mut u8 = (SD0_CONF_REG_BASEADDRESS + reg) as *mut u8;
    unsafe {
        return sdio_reg.read_volatile();
    }
}

#[inline(always)]
pub unsafe fn sdio1_reg_read_u8(reg: u32) -> u8 {
    let sdio_reg : *mut u8 = (SD1_CONF_REG_BASEADDRESS + reg) as *mut u8;
    unsafe {
        return sdio_reg.read_volatile();
    }
}

#[inline(always)]
pub fn sdio_get_cntrlr_vers(sd_card_id : SDCardId) -> u16 {
    let cntrl_vers: u16;
    unsafe{
       cntrl_vers = sdio_reg_read_u16(sd_card_id, SDIO_REG_HOSTCONTROLLERVER_REG_OFFSET);
    }
    return cntrl_vers;
}

#[inline(always)]
pub fn sdio0_get_cntrlr_vers() -> u16 {
    let cntrl_vers: u16;
    unsafe{
       cntrl_vers = sdio0_reg_read_u16(SDIO_REG_HOSTCONTROLLERVER_REG_OFFSET);
    }
    return cntrl_vers;
}

#[inline(always)]
pub fn sdio1_get_cntrlr_vers() -> u16 {
    let cntrl_vers: u16;
    unsafe{
       cntrl_vers = sdio1_reg_read_u16(SDIO_REG_HOSTCONTROLLERVER_REG_OFFSET);
    }
    return cntrl_vers;
}

#[inline(always)]
pub fn sdio_software_reset_all(sd_card_id : SDCardId) {
    unsafe{
       sdio_reg_write_u8(sd_card_id, SDIO_REG_SOFTWARERESET_REG_OFFSET, SDIO_REG_SOFTWARERESET_SWRESET_FOR_ALL_MASK);
    }
}

#[inline(always)]
pub fn sdio_all_in_reset(sd_card_id : SDCardId) -> bool {
    let ret_val : u8;
    unsafe{
       ret_val = sdio_reg_read_u8(sd_card_id, SDIO_REG_SOFTWARERESET_REG_OFFSET);
    }
    (ret_val & SDIO_REG_SOFTWARERESET_SWRESET_FOR_ALL_MASK) != 0
}

#[inline(always)]
pub fn sdio_read_capabilities(sd_card_id : SDCardId) -> u64 {
    unsafe{
       sdio_reg_read_u64(sd_card_id, SDIO_REG_CAPABILITIES_REG_OFFSET)
    }
}

#[inline(always)]
pub fn sdio_set_power_cntrl_default(sd_card_id : SDCardId) {
    const DEFAULT_PWR_SETTINGS : u8 = (POWERCONTROL_VOLTAGE_3V3 << SDIO_REG_POWERCONTROL_PWCTRL_SDBUSVOLTAGE_OFF) | POWERCONTROL_POWER_ON;
    unsafe{
       sdio_reg_write_u8(sd_card_id, SDIO_REG_POWERCONTROL_REG_OFFSET, DEFAULT_PWR_SETTINGS);
    }
}


pub fn sdio_set_clk_cntrl_default(sd_card_id : SDCardId) {
    // disable all clocks (other values can be set to 0 as well since they all depend on the clocks being enabled, so they do nothing when clocks disabled)
    unsafe{
       sdio_reg_write_u16(sd_card_id, SDIO_REG_CLOCKCONTROL_REG_OFFSET, 0);
    }
    // Internal clock needs to be cleared for 1 clock cycle whenever the frequency is changed (meaning write disable then write (enable, new freq))
    const CLK_DIV : u16 = 0; // TODO: Calculate this for 400 KHz. This is the divisor value that should give 400 KHz = Base clk / CLK_DIV. Where do I get Base clk?
    const CLK_DEFAULT : u16 = ( CLK_DIV << SDIO_REG_CLOCKCONTROL_CLKCTRL_SDCLKFREQSEL_OFF ) | CLOCKCONTROL_HC_CLK_ENABLE;
    unsafe{
        sdio_reg_write_u16(sd_card_id, SDIO_REG_CLOCKCONTROL_REG_OFFSET, CLK_DEFAULT);
    }

    // Poll the Dsync bit until we are ready
    let mut clk_cntrl_val : u16;
    unsafe{
       clk_cntrl_val = sdio_reg_read_u16(sd_card_id, SDIO_REG_CLOCKCONTROL_REG_OFFSET);
    }
    // TODO: Time out?
    //        Documentation says this is 1 to 2 us. Is this long enough that we should go do something else and comeback?
    while (clk_cntrl_val & SDIO_REG_CLOCKCONTROL_SDHCCLKGEN_INTCLKSTABLE_DSYNC_MASK) != 0 {
        unsafe{
            clk_cntrl_val = sdio_reg_read_u16(sd_card_id, SDIO_REG_CLOCKCONTROL_REG_OFFSET);
        }
    }

    let sd_clk_en = clk_cntrl_val | CLOCKCONTROL_SD_CLK_ENABLE;
    unsafe{
        sdio_reg_write_u16(sd_card_id, SDIO_REG_CLOCKCONTROL_REG_OFFSET, sd_clk_en);
    }
}

/* 
#[inline(always)]
pub fn sdio_perform_dma_read(sd_card_id : SDCardId, ) {
    let cntrl_vers: u16;
    unsafe{
       cntrl_vers = sdio_reg_read_u16(sd_card_id, SDIO_REG_HOSTCONTROLLERVER_REG_OFFSET);
    }
    return cntrl_vers;
}*/
