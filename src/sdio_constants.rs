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


#[repr(u8)]
#[derive(PartialEq)]
pub enum SDCardId {
    SD0 = 0,
    SD1 = 1,
}

#[inline(always)]
pub unsafe fn sdio_reg_write(sd_card_id : SDCardId, reg: u32, val: u32) -> () {
    let sdio_reg : *mut u32 = (SD0_CONF_REG_BASEADDRESS + ( (sd_card_id as u32) << 16 ) + reg) as *mut u32;
    unsafe {
        sdio_reg.write_volatile(val);
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
pub fn sdio_get_cntrlr_vers(sd_card_id : SDCardId) -> u16 {
    let cntrl_vers: u16;
    unsafe{
       cntrl_vers = sdio_reg_read_u16(SDIO_REG_HOSTCONTROLLERVER_REG_OFFSET);
    }
    return cntrl_vers;
}
