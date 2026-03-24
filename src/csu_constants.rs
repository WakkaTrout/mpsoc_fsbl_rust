//CSU Config Register Baseaddress
const CSU_CONF_REG_BASEADDRESS           : u32 = 0xFFCA0000;

// CSU Register Offsets
pub const CSU_STATUS_REG_OFFSET              : u32 = 0x00000000;
pub const CSU_CTRL_REG_OFFSET                : u32 = 0x00000004;
pub const CSU_SSS_CFG_REG_OFFSET             : u32 = 0x00000008;
pub const CSU_DMA_RESET_REG_OFFSET           : u32 = 0x0000000C;
pub const CSU_MULTIBOOT_REG_OFFSET           : u32 = 0x00000010;
pub const CSU_TAMPER_TRIG_REG_OFFSET         : u32 = 0x00000014;
pub const CSU_FT_STATUS_REG_OFFSET           : u32 = 0x00000018;

pub const CSU_ISR_REG_OFFSET                 : u32 = 0x00000020;
pub const CSU_IMR_REG_OFFSET                 : u32 = 0x00000024;
pub const CSU_IER_REG_OFFSET                 : u32 = 0x00000028;
pub const CSU_IDR_REG_OFFSET                 : u32 = 0x0000002C;
pub const CSU_JTAG_CHAIN_CFG_REG_OFFSET      : u32 = 0x00000030;
pub const CSU_JTAG_CHAIN_STATUS_REG_OFFSET   : u32 = 0x00000034;
pub const CSU_JTAG_SEC_REG_OFFSET            : u32 = 0x00000038;
pub const CSU_JTAG_DAP_REG_OFFSET            : u32 = 0x0000003C;
pub const CSU_IDCODE_REG_OFFSET              : u32 = 0x00000040;
pub const CSU_VERSION_REG_OFFSET             : u32 = 0x00000044;

pub const CSU_ROM_DIGEST_ADDR_00_REG_OFFSET  : u32 = 0x00000050;
pub const CSU_ROM_DIGEST_ADDR_01_REG_OFFSET  : u32 = 0x00000054;
pub const CSU_ROM_DIGEST_ADDR_02_REG_OFFSET  : u32 = 0x00000058;
pub const CSU_ROM_DIGEST_ADDR_03_REG_OFFSET  : u32 = 0x0000005C;
pub const CSU_ROM_DIGEST_ADDR_04_REG_OFFSET  : u32 = 0x00000060;
pub const CSU_ROM_DIGEST_ADDR_05_REG_OFFSET  : u32 = 0x00000064;
pub const CSU_ROM_DIGEST_ADDR_06_REG_OFFSET  : u32 = 0x00000068;
pub const CSU_ROM_DIGEST_ADDR_07_REG_OFFSET  : u32 = 0x0000006C;
pub const CSU_ROM_DIGEST_ADDR_08_REG_OFFSET  : u32 = 0x00000070;
pub const CSU_ROM_DIGEST_ADDR_09_REG_OFFSET  : u32 = 0x00000074;
pub const CSU_ROM_DIGEST_ADDR_10_REG_OFFSET  : u32 = 0x00000078;
pub const CSU_ROM_DIGEST_ADDR_11_REG_OFFSET  : u32 = 0x0000007C;

pub const CSU_AES_STATUS_REG_OFFSET          : u32 = 0x00001000;
pub const CSU_AES_KEY_SRC_REG_OFFSET         : u32 = 0x00001004;
pub const CSU_AES_KEY_LOAD_REG_OFFSET        : u32 = 0x00001008;
pub const CSU_AES_START_MSG_REG_OFFSET       : u32 = 0x0000100C;
pub const CSU_AES_RESET_REG_OFFSET           : u32 = 0x00001010;
pub const CSU_AES_KEY_CLR_REG_OFFSET         : u32 = 0x00001014;

pub const CSU_SHA_START_REG_OFFSET           : u32 = 0x00002000;
pub const CSU_SHA_RESET_REG_OFFSET           : u32 = 0x00002004;
pub const CSU_SHA_DONE_REG_OFFSET            : u32 = 0x00002008;
pub const CSU_SHA_DIGEST_0_REG_OFFSET        : u32 = 0x00002010;
pub const CSU_PCAP_PROG_REG_OFFSET           : u32 = 0x00003000;
pub const CSU_PCAP_RDWR_REG_OFFSET           : u32 = 0x00003004;
pub const CSU_PCAP_CTRL_REG_OFFSET           : u32 = 0x00003008;
pub const CSU_PCAP_RESET_REG_OFFSET          : u32 = 0x0000300C;
pub const CSU_PCAP_STATUS_REG_OFFSET         : u32 = 0x00003010;

//CSU Register Masks

#[inline(always)]
pub unsafe fn csu_reg_write(reg: u32, val: u32) {
    let csu_reg : *mut u32 = (reg + CSU_CONF_REG_BASEADDRESS) as *mut u32;
    unsafe {
        csu_reg.write_volatile(val);
    }
}

#[inline(always)]
pub fn csu_aes_engine_reset() {
    unsafe {
        csu_reg_write(CSU_AES_RESET_REG_OFFSET,0);
    }
}