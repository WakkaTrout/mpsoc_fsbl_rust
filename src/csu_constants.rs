// Cryptographic Security Unit (CSU) Configuration Registers

// All Values below are from UG 1087 (https://docs.amd.com/r/en-US/ug1087-zynq-ultrascale-registers)

//CSU Config Register Baseaddress
const CSU_CONF_REG_BASEADDRESS               : u32 = 0xFFCA0000;

// CSU Register Offsets
const CSU_STATUS_REG_OFFSET              : u32 = 0x00000000;
const CSU_CTRL_REG_OFFSET                : u32 = 0x00000004;
const CSU_SSS_CFG_REG_OFFSET             : u32 = 0x00000008;
const CSU_DMA_RESET_REG_OFFSET           : u32 = 0x0000000C;
const CSU_MULTIBOOT_REG_OFFSET           : u32 = 0x00000010;
const CSU_TAMPER_TRIG_REG_OFFSET         : u32 = 0x00000014;
const CSU_FT_STATUS_REG_OFFSET           : u32 = 0x00000018;

const CSU_ISR_REG_OFFSET                 : u32 = 0x00000020;
const CSU_IMR_REG_OFFSET                 : u32 = 0x00000024;
const CSU_IER_REG_OFFSET                 : u32 = 0x00000028;
const CSU_IDR_REG_OFFSET                 : u32 = 0x0000002C;
const CSU_JTAG_CHAIN_CFG_REG_OFFSET      : u32 = 0x00000030;
const CSU_JTAG_CHAIN_STATUS_REG_OFFSET   : u32 = 0x00000034;
const CSU_JTAG_SEC_REG_OFFSET            : u32 = 0x00000038;
const CSU_JTAG_DAP_REG_OFFSET            : u32 = 0x0000003C;
const CSU_IDCODE_REG_OFFSET              : u32 = 0x00000040;
const CSU_VERSION_REG_OFFSET             : u32 = 0x00000044;


const CSU_ROM_DIGEST_ADDR_00_REG_OFFSET  : u32 = 0x00000050;
const CSU_ROM_DIGEST_ADDR_01_REG_OFFSET  : u32 = 0x00000054;
const CSU_ROM_DIGEST_ADDR_02_REG_OFFSET  : u32 = 0x00000058;
const CSU_ROM_DIGEST_ADDR_03_REG_OFFSET  : u32 = 0x0000005C;
const CSU_ROM_DIGEST_ADDR_04_REG_OFFSET  : u32 = 0x00000060;
const CSU_ROM_DIGEST_ADDR_05_REG_OFFSET  : u32 = 0x00000064;
const CSU_ROM_DIGEST_ADDR_06_REG_OFFSET  : u32 = 0x00000068;
const CSU_ROM_DIGEST_ADDR_07_REG_OFFSET  : u32 = 0x0000006C;
const CSU_ROM_DIGEST_ADDR_08_REG_OFFSET  : u32 = 0x00000070;
const CSU_ROM_DIGEST_ADDR_09_REG_OFFSET  : u32 = 0x00000074;
const CSU_ROM_DIGEST_ADDR_10_REG_OFFSET  : u32 = 0x00000078;
const CSU_ROM_DIGEST_ADDR_11_REG_OFFSET  : u32 = 0x0000007C;


const CSU_AES_STATUS_REG_OFFSET          : u32 = 0x00001000;
const CSU_AES_KEY_SRC_REG_OFFSET         : u32 = 0x00001004;
const CSU_AES_KEY_LOAD_REG_OFFSET        : u32 = 0x00001008;
const CSU_AES_START_MSG_REG_OFFSET       : u32 = 0x0000100C;
const CSU_AES_RESET_REG_OFFSET           : u32 = 0x00001010;
const CSU_AES_KEY_CLR_REG_OFFSET         : u32 = 0x00001014;
const CSU_AES_CFG_REG_OFFSET             : u32 = 0x00001018;
const CSU_AES_KUP_WR_REG_OFFSET          : u32 = 0x0000101C;
const CSU_AES_KUP_0_REG_OFFSET           : u32 = 0x00001020;
const CSU_AES_KUP_1_REG_OFFSET           : u32 = 0x00001024;
const CSU_AES_KUP_2_REG_OFFSET           : u32 = 0x00001028;
const CSU_AES_KUP_3_REG_OFFSET           : u32 = 0x0000102C;
const CSU_AES_KUP_4_REG_OFFSET           : u32 = 0x00001030;
const CSU_AES_KUP_5_REG_OFFSET           : u32 = 0x00001034;
const CSU_AES_KUP_6_REG_OFFSET           : u32 = 0x00001038;
const CSU_AES_KUP_7_REG_OFFSET           : u32 = 0x0000103C;
const CSU_AES_IV_0_REG_OFFSET            : u32 = 0x00001040;
const CSU_AES_IV_1_REG_OFFSET            : u32 = 0x00001044;
const CSU_AES_IV_2_REG_OFFSET            : u32 = 0x00001048;
const CSU_AES_IV_3_REG_OFFSET            : u32 = 0x0000104C;


const CSU_SHA_START_REG_OFFSET           : u32 = 0x00002000;
const CSU_SHA_RESET_REG_OFFSET           : u32 = 0x00002004;
const CSU_SHA_DONE_REG_OFFSET            : u32 = 0x00002008;

const CSU_SHA_DIGEST_00_REG_OFFSET       : u32 = 0x00002010;
const CSU_SHA_DIGEST_01_REG_OFFSET       : u32 = 0x00002014;
const CSU_SHA_DIGEST_02_REG_OFFSET       : u32 = 0x00002018;
const CSU_SHA_DIGEST_03_REG_OFFSET       : u32 = 0x0000201C;
const CSU_SHA_DIGEST_04_REG_OFFSET       : u32 = 0x00002020;
const CSU_SHA_DIGEST_05_REG_OFFSET       : u32 = 0x00002024;
const CSU_SHA_DIGEST_06_REG_OFFSET       : u32 = 0x00002028;
const CSU_SHA_DIGEST_07_REG_OFFSET       : u32 = 0x0000202C;
const CSU_SHA_DIGEST_08_REG_OFFSET       : u32 = 0x00002030;
const CSU_SHA_DIGEST_09_REG_OFFSET       : u32 = 0x00002034;
const CSU_SHA_DIGEST_10_REG_OFFSET       : u32 = 0x00002038;
const CSU_SHA_DIGEST_11_REG_OFFSET       : u32 = 0x0000203C;


const CSU_PCAP_PROG_REG_OFFSET           : u32 = 0x00003000;
const CSU_PCAP_RDWR_REG_OFFSET           : u32 = 0x00003004;
const CSU_PCAP_CTRL_REG_OFFSET           : u32 = 0x00003008;
const CSU_PCAP_RESET_REG_OFFSET          : u32 = 0x0000300C;
const CSU_PCAP_STATUS_REG_OFFSET         : u32 = 0x00003010;


const CSU_PUF_CMD_REG_OFFSET             : u32 = 0x00004000;
const CSU_PUF_CFG0_REG_OFFSET            : u32 = 0x00004004;
const CSU_PUF_CFG1_REG_OFFSET            : u32 = 0x00004008;
const CSU_PUF_SHUT_REG_OFFSET            : u32 = 0x0000400C;
const CSU_PUF_STATUS_REG_OFFSET          : u32 = 0x00004010;
const CSU_PUF_DBG_REG_OFFSET             : u32 = 0x00004014;
const CSU_PUF_WORD_REG_OFFSET            : u32 = 0x00004018;


const CSU_PUF_TM_STATUS_REG_OFFSET       : u32 = 0x00004804;
const CSU_PUF_TM_UL_REG_OFFSET           : u32 = 0x00004808;
const CSU_PUF_TM_LL_REG_OFFSET           : u32 = 0x0000480C;
const CSU_PUF_TM_SW_REG_OFFSET           : u32 = 0x00004810;
const CSU_PUF_TM_TR_REG_OFFSET           : u32 = 0x00004814;


const CSU_TAMPER_STATUS_REG_OFFSET       : u32 = 0x00005000;
const CSU_TAMPER_00_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_01_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_02_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_03_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_04_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_05_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_06_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_07_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_08_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_09_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_10_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_11_REG_OFFSET           : u32 = 0x00005004;
const CSU_TAMPER_12_REG_OFFSET           : u32 = 0x00005004;

//CSU Register Constants

pub const CSU_STATUS_BOOT_ENC_MASK           : u32 = 0x00000002;
pub const CSU_STATUS_BOOT_ENC_OFF            : u32 = 1;
pub const CSU_STATUS_BOOT_AUTH_MASK          : u32 = 0x00000001;
pub const CSU_STATUS_BOOT_AUTH_OFF           : u32 = 0;

pub const CSU_CTRL_SLVERR_ENABLE_MASK        : u32 = 0x00000010;
pub const CSU_CTRL_SLVERR_ENABLE_OFF         : u32 = 4;
pub const CSU_CTRL_CSU_CLK_SEL_MASK          : u32 = 0x00000001;
pub const CSU_CTRL_CSU_CLK_SEL_OFF           : u32 = 0;

pub const CSU_SSS_CFG_SHA_SSS_MASK           : u32 = 0x0000F000;
pub const CSU_SSS_CFG_SHA_SSS_OFF            : u32 = 12;
pub const CSU_SSS_CFG_AES_SSS_MASK           : u32 = 0x00000F00;
pub const CSU_SSS_CFG_AES_SSS_OFF            : u32 = 8;
pub const CSU_SSS_CFG_DMA_SSS_MASK           : u32 = 0x000000F0;
pub const CSU_SSS_CFG_DMA_SSS_OFF            : u32 = 4;
pub const CSU_SSS_CFG_PCAP_SSS_MASK          : u32 = 0x0000000F;
pub const CSU_SSS_CFG_PCAP_SSS_OFF           : u32 = 0;

pub const CSU_DMA_RESET_RESET_MASK           : u32 = 0x00000001;
pub const CSU_DMA_RESET_RESET_OFF            : u32 = 0;

pub const CSU_MULTIBOOT_SSSS_ADDR_MASK       : u32 = 0xFFFFFFFF;
pub const CSU_MULTIBOOT_SSSS_ADDR_OFF        : u32 = 0;

pub const CSU_TAMPER_TRIG_TRIGGER_MASK       : u32 = 0x00000001;
pub const CSU_TAMPER_TRIG_TRIGGER_OFF        : u32 = 0;

pub const CSU_AES_RESET_RESET_MASK           : u32 = 0x00000001;
pub const CSU_AES_RESET_RESET_OFF            : u32 = 0;

pub const CSU_SHA_RESET_RESET_MASK           : u32 = 0x00000001;
pub const CSU_SHA_RESET_RESET_OFF            : u32 = 0;

// CSU Functions

#[inline(always)]
pub unsafe fn csu_reg_write(reg: u32, val: u32) {
    let csu_reg : *mut u32 = (reg + CSU_CONF_REG_BASEADDRESS) as *mut u32;
    unsafe {
        csu_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn csu_reg_read(reg: u32) -> u32 {
    let csu_reg : *mut u32 = (reg + CSU_CONF_REG_BASEADDRESS) as *mut u32;
    unsafe {
        return csu_reg.read_volatile();
    }
}


#[inline(always)]
pub fn csu_aes_engine_reset() {
    unsafe {
        csu_reg_write(CSU_AES_RESET_REG_OFFSET,CSU_AES_RESET_RESET_MASK);
    }
}

#[inline(always)]
pub fn csu_sha_engine_reset() {
    unsafe {
        csu_reg_write(CSU_SHA_RESET_REG_OFFSET,CSU_SHA_RESET_RESET_MASK);
    }
}
