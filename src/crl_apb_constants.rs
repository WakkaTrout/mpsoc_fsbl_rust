// CRL_APB
// XX XX LPD   XX XX XX
// Low Power Domain (LPD) clock and reset control

// All Values below are from UG 1087 (https://docs.amd.com/r/en-US/ug1087-zynq-ultrascale-registers)

// CRL APB Config Register Baseaddress
const CRL_APB_REG_BASEADDRESS                   : u32 = 0xFF5E0000;

// CRL APB Register Offsets
pub const CRL_APB_ERR_CTRL_REG_OFFSET           : u32 = 0x00000000;
pub const CRL_APB_IR_STATUS_REG_OFFSET          : u32 = 0x00000004;
pub const CRL_APB_IR_MASK_REG_OFFSET            : u32 = 0x00000008;
pub const CRL_APB_IR_ENABLE_REG_OFFSET          : u32 = 0x0000000C;
pub const CRL_APB_IR_DISABLE_REG_OFFSET         : u32 = 0x00000010;


pub const CRL_APB_CRL_WRPROT_REG_OFFSET         : u32 = 0x0000001C;
pub const CRL_APB_IOPLL_CTRL_REG_OFFSET         : u32 = 0x00000020;
pub const CRL_APB_IOPLL_CFG_REG_OFFSET          : u32 = 0x00000024;
pub const CRL_APB_IOPLL_FRAC_CFG_REG_OFFSET     : u32 = 0x00000028;

pub const CRL_APB_RPLL_CTRL_REG_OFFSET          : u32 = 0x00000030;
pub const CRL_APB_RPLL_CFG_REG_OFFSET           : u32 = 0x00000034;
pub const CRL_APB_RPLL_FRAC_CFG_REG_OFFSET      : u32 = 0x00000038;

pub const CRL_APB_PLL_STATUS_REG_OFFSET         : u32 = 0x00000040;
pub const CRL_APB_IOPLL_TO_FPD_CTRL_REG_OFFSET  : u32 = 0x00000044;
pub const CRL_APB_RPLL_TO_FPD_CTRL_REG_OFFSET   : u32 = 0x00000048;
pub const CRL_APB_USB3_DUAL_REF_CTRL_REG_OFFSET : u32 = 0x0000004C;
pub const CRL_APB_GEM0_CTRL_REG_OFFSET          : u32 = 0x00000050;
pub const CRL_APB_GEM1_CTRL_REG_OFFSET          : u32 = 0x00000054;
pub const CRL_APB_GEM2_CTRL_REG_OFFSET          : u32 = 0x00000058;
pub const CRL_APB_GEM3_CTRL_REG_OFFSET          : u32 = 0x0000005C;
pub const CRL_APB_USB0_BUS_REF_CTRL_REG_OFFSET  : u32 = 0x00000060;
pub const CRL_APB_USB1_BUS_REF_CTRL_REG_OFFSET  : u32 = 0x00000064;
pub const CRL_APB_QSPI_REF_CTRL_REG_OFFSET      : u32 = 0x00000068;
pub const CRL_APB_SDIO0_REF_CTRL_REG_OFFSET     : u32 = 0x0000006C;
pub const CRL_APB_SDIO1_REF_CTRL_REG_OFFSET     : u32 = 0x00000070;
pub const CRL_APB_UART0_REF_CTRL_REG_OFFSET     : u32 = 0x00000074;
pub const CRL_APB_UART1_REF_CTRL_REG_OFFSET     : u32 = 0x00000078;
pub const CRL_APB_SPI0_REF_CTRL_REG_OFFSET      : u32 = 0x0000007C;
pub const CRL_APB_SPI1_REF_CTRL_REG_OFFSET      : u32 = 0x00000080;
pub const CRL_APB_CAN0_REF_CTRL_REG_OFFSET      : u32 = 0x00000084;
pub const CRL_APB_CAN1_REF_CTRL_REG_OFFSET      : u32 = 0x00000088;

pub const CRL_APB_CPU_R5_CTRL_REG_OFFSET        : u32 = 0x00000090;


pub const CRL_APB_IOU_SWITCH_CTRL_REG_OFFSET    : u32 = 0x0000009C;
pub const CRL_APB_CSU_PLL_CTRL_REG_OFFSET       : u32 = 0x000000A0;
pub const CRL_APB_PCAP_CTRL_REG_OFFSET          : u32 = 0x000000A4;
pub const CRL_APB_LPD_SWITCH_CTRL_REG_OFFSET    : u32 = 0x000000A8;
pub const CRL_APB_LPD_LSBUS_CTRL_REG_OFFSET     : u32 = 0x000000AC;
pub const CRL_APB_DBG_LPD_CTRL_REG_OFFSET       : u32 = 0x000000B0;
pub const CRL_APB_NAND_REF_CTRL_REG_OFFSET      : u32 = 0x000000B4;
pub const CRL_APB_LPD_DMA_REF_CTRL_REG_OFFSET   : u32 = 0x000000B8;

pub const CRL_APB_PL0_REF_CTRL_REG_OFFSET       : u32 = 0x000000C0;
pub const CRL_APB_PL1_REF_CTRL_REG_OFFSET       : u32 = 0x000000C4;
pub const CRL_APB_PL2_REF_CTRL_REG_OFFSET       : u32 = 0x000000C8;
pub const CRL_APB_PL3_REF_CTRL_REG_OFFSET       : u32 = 0x000000CC;
pub const CRL_APB_PL0_THR_CTRL_REG_OFFSET       : u32 = 0x000000D0;
pub const CRL_APB_PL0_THR_CNT_REG_OFFSET        : u32 = 0x000000D4;
pub const CRL_APB_PL1_THR_CTRL_REG_OFFSET       : u32 = 0x000000D8;
pub const CRL_APB_PL1_THR_CNT_REG_OFFSET        : u32 = 0x000000DC;
pub const CRL_APB_PL2_THR_CTRL_REG_OFFSET       : u32 = 0x000000E0;
pub const CRL_APB_PL2_THR_CNT_REG_OFFSET        : u32 = 0x000000E4;
pub const CRL_APB_PL3_THR_CTRL_REG_OFFSET       : u32 = 0x000000E8;
pub const CRL_APB_PL3_THR_CNT_REG_OFFSET        : u32 = 0x000000FC; // ??? Is this correct, the documentation says this, but it doesn't look right??


pub const CRL_APB_GEM_TSU_REF_CTRL_REG_OFFSET   : u32 = 0x00000100;
pub const CRL_APB_DLL_REF_CTRL_REG_OFFSET       : u32 = 0x00000104;
pub const CRL_APB_PSSYSMON_REF_CTRL_REG_OFFSET  : u32 = 0x00000108;


pub const CRL_APB_I2C0_REF_CTRL_REG_OFFSET      : u32 = 0x00000120;
pub const CRL_APB_I2C1_REF_CTRL_REG_OFFSET      : u32 = 0x00000124;
pub const CRL_APB_TIMESTAMP_REF_CTRL_REG_OFFSET : u32 = 0x00000128;

pub const CRL_APB_SAFETY_CHK_REG_OFFSET         : u32 = 0x00000130;


pub const CRL_APB_CLKMON_STATUS_REG_OFFSET      : u32 = 0x00000140;
pub const CRL_APB_CLKMON_MASK_REG_OFFSET        : u32 = 0x00000144;
pub const CRL_APB_CLKMON_ENABLE_REG_OFFSET      : u32 = 0x00000148;
pub const CRL_APB_CLKMON_DISABLE_REG_OFFSET     : u32 = 0x0000014C;
pub const CRL_APB_CLKMON_TRIGGER_REG_OFFSET     : u32 = 0x00000150;


pub const CRL_APB_CHKR0_CLKA_UPPER_REG_OFFSET   : u32 = 0x00000160;
pub const CRL_APB_CHKR0_CLKA_LOWER_REG_OFFSET   : u32 = 0x00000164;
pub const CRL_APB_CHKR0_CLKB_CNT_REG_OFFSET     : u32 = 0x00000168;
pub const CRL_APB_CHKR0_CTRL_REG_OFFSET         : u32 = 0x0000016C;
pub const CRL_APB_CHKR1_CLKA_UPPER_REG_OFFSET   : u32 = 0x00000170;
pub const CRL_APB_CHKR1_CLKA_LOWER_REG_OFFSET   : u32 = 0x00000174;
pub const CRL_APB_CHKR1_CLKB_CNT_REG_OFFSET     : u32 = 0x00000178;
pub const CRL_APB_CHKR1_CTRL_REG_OFFSET         : u32 = 0x0000017C;
pub const CRL_APB_CHKR2_CLKA_UPPER_REG_OFFSET   : u32 = 0x00000180;
pub const CRL_APB_CHKR2_CLKA_LOWER_REG_OFFSET   : u32 = 0x00000184;
pub const CRL_APB_CHKR2_CLKB_CNT_REG_OFFSET     : u32 = 0x00000188;
pub const CRL_APB_CHKR2_CTRL_REG_OFFSET         : u32 = 0x0000018C;
pub const CRL_APB_CHKR3_CLKA_UPPER_REG_OFFSET   : u32 = 0x00000190;
pub const CRL_APB_CHKR3_CLKA_LOWER_REG_OFFSET   : u32 = 0x00000194;
pub const CRL_APB_CHKR3_CLKB_CNT_REG_OFFSET     : u32 = 0x00000198;
pub const CRL_APB_CHKR3_CTRL_REG_OFFSET         : u32 = 0x0000019C;
pub const CRL_APB_CHKR4_CLKA_UPPER_REG_OFFSET   : u32 = 0x000001A0;
pub const CRL_APB_CHKR4_CLKA_LOWER_REG_OFFSET   : u32 = 0x000001A4;
pub const CRL_APB_CHKR4_CLKB_CNT_REG_OFFSET     : u32 = 0x000001A8;
pub const CRL_APB_CHKR4_CTRL_REG_OFFSET         : u32 = 0x000001AC;
pub const CRL_APB_CHKR5_CLKA_UPPER_REG_OFFSET   : u32 = 0x000001B0;
pub const CRL_APB_CHKR5_CLKA_LOWER_REG_OFFSET   : u32 = 0x000001B4;
pub const CRL_APB_CHKR5_CLKB_CNT_REG_OFFSET     : u32 = 0x000001B8;
pub const CRL_APB_CHKR5_CTRL_REG_OFFSET         : u32 = 0x000001BC;
pub const CRL_APB_CHKR6_CLKA_UPPER_REG_OFFSET   : u32 = 0x000001C0;
pub const CRL_APB_CHKR6_CLKA_LOWER_REG_OFFSET   : u32 = 0x000001C4;
pub const CRL_APB_CHKR6_CLKB_CNT_REG_OFFSET     : u32 = 0x000001C8;
pub const CRL_APB_CHKR6_CTRL_REG_OFFSET         : u32 = 0x000001CC;
pub const CRL_APB_CHKR7_CLKA_UPPER_REG_OFFSET   : u32 = 0x000001D0;
pub const CRL_APB_CHKR7_CLKA_LOWER_REG_OFFSET   : u32 = 0x000001D4;
pub const CRL_APB_CHKR7_CLKB_CNT_REG_OFFSET     : u32 = 0x000001D8;
pub const CRL_APB_CHKR7_CTRL_REG_OFFSET         : u32 = 0x000001DC;


pub const CRL_APB_BOOT_MODE_USER_REG_OFFSET     : u32 = 0x00000200;
pub const CRL_APB_BOOT_MODE_POR_REG_OFFSET      : u32 = 0x00000204;


pub const CRL_APB_RESET_CTRL_REG_OFFSET         : u32 = 0x00000218;
pub const CRL_APB_BLOCKONLY_RST_REG_OFFSET      : u32 = 0x0000021C;
pub const CRL_APB_RESET_REASON_REG_OFFSET       : u32 = 0x00000220;


pub const CRL_APB_RST_LPD_IOU0_REG_OFFSET       : u32 = 0x00000230;
// No IUO1?
pub const CRL_APB_RST_LPD_IOU2_REG_OFFSET       : u32 = 0x00000238;
pub const CRL_APB_RST_LPD_TOP_REG_OFFSET        : u32 = 0x0000023C;
pub const CRL_APB_RST_LPD_DBG_REG_OFFSET        : u32 = 0x00000240;


pub const CRL_APB_BOOT_PIN_CTRL_REG_OFFSET      : u32 = 0x00000250;

// Other banks?
pub const CRL_APB_BANK3_CTRL0_REG_OFFSET        : u32 = 0x00000270;
pub const CRL_APB_BANK3_CTRL1_REG_OFFSET        : u32 = 0x00000274;
pub const CRL_APB_BANK3_CTRL2_REG_OFFSET        : u32 = 0x00000278;
pub const CRL_APB_BANK3_CTRL3_REG_OFFSET        : u32 = 0x0000027C;
pub const CRL_APB_BANK3_CTRL4_REG_OFFSET        : u32 = 0x00000280;
pub const CRL_APB_BANK3_CTRL5_REG_OFFSET        : u32 = 0x00000284;
pub const CRL_APB_BANK3_STATUS_REG_OFFSET       : u32 = 0x00000288;

// CRL APB Register Constants

pub const CRL_APB_BOOT_MODE_USER_ALT_BOOT_MODE_MASK  : u32 = 0x0000F000;
pub const CRL_APB_BOOT_MODE_USER_ALT_BOOT_MODE_OFF   : u32 = 12;
pub const CRL_APB_BOOT_MODE_USER_USE_ALT_MASK        : u32 = 0x00000100;
pub const CRL_APB_BOOT_MODE_USER_USE_ALT_OFF         : u32 = 8;
pub const CRL_APB_BOOT_MODE_USER_BOOT_MODE_MASK      : u32 = 0x0000000F;
pub const CRL_APB_BOOT_MODE_USER_BOOT_MODE_OFF       : u32 = 0;

#[repr(u8)]
#[derive(PartialEq)]
pub enum BootMode {
    JtagBootMode   = 0,
    Qspi24BootMode = 1,
    Qspi32BootMode = 2,
    Sd0BootMode = 3,
    NandBootMode = 4,
    Sd1BootMode = 5,
    EmmcBootMode = 6,
    UsbBootMode = 7,
    Sd1LSBootMode = 14,
    UndefinedBootMode = 255
}

pub const CRL_APB_RESET_REASON_DEBUG_SYS_MASK        : u32 = 0x00000040; // Software Debugger Reset
pub const CRL_APB_RESET_REASON_DEBUG_SYS_OFF         : u32 = 6;
pub const CRL_APB_RESET_REASON_SOFT_MASK             : u32 = 0x00000020; // Soft Reset
pub const CRL_APB_RESET_REASON_SOFT_OFF              : u32 = 5;
pub const CRL_APB_RESET_REASON_SRST_MASK             : u32 = 0x00000010; // External System Reset
pub const CRL_APB_RESET_REASON_SRST_OFF              : u32 = 4;
pub const CRL_APB_RESET_REASON_PSONLY_RESET_REQ_MASK : u32 = 0x00000008; // PS Only Reset
pub const CRL_APB_RESET_REASON_PSONLY_RESET_REQ_OFF  : u32 = 3;
pub const CRL_APB_RESET_REASON_PMU_SYS_RESET_MASK    : u32 = 0x00000004; // Internal System Reset
pub const CRL_APB_RESET_REASON_PMU_SYS_RESET_OFF     : u32 = 2;
pub const CRL_APB_RESET_REASON_INTERNAL_POR_MASK     : u32 = 0x00000002; // Internal POR
pub const CRL_APB_RESET_REASON_INTERNAL_POR_OFF      : u32 = 1;
pub const CRL_APB_RESET_REASON_EXTERNAL_POR_MASK     : u32 = 0x00000001; // External POR
pub const CRL_APB_RESET_REASON_EXTERNAL_POR_OFF      : u32 = 0;

#[inline(always)]
pub unsafe fn crl_apb_reg_write(reg: u32, val: u32) -> () {
    let crl_apb_reg : *mut u32 = (reg + CRL_APB_REG_BASEADDRESS) as *mut u32;
    unsafe {
        crl_apb_reg.write_volatile(val);
    }
}

#[inline(always)]
pub unsafe fn crl_apb_reg_read(reg: u32) -> u32 {
    let crl_apb_reg : *mut u32 = (reg + CRL_APB_REG_BASEADDRESS) as *mut u32;
    unsafe {
        return crl_apb_reg.read_volatile();
    }
}

#[inline(always)]
pub fn crl_apb_get_reset_reason() -> u32 {
    let reset_reason : u32;
    unsafe {
        reset_reason = crl_apb_reg_read(CRL_APB_RESET_REASON_REG_OFFSET);
    }
    return reset_reason;
}

#[inline(always)]
pub fn crl_apb_get_user_boot_mode() -> BootMode {
    let boot_mode : u32;
    unsafe {
        boot_mode = crl_apb_reg_read(CRL_APB_BOOT_MODE_USER_REG_OFFSET);
    }
    match (boot_mode & CRL_APB_BOOT_MODE_USER_BOOT_MODE_MASK) >> CRL_APB_BOOT_MODE_USER_BOOT_MODE_OFF
    {
        0=> BootMode::JtagBootMode,
        1=> BootMode::Qspi24BootMode,
        2=> BootMode::Qspi32BootMode,
        3=> BootMode::Sd0BootMode,
        4=> BootMode::NandBootMode,
        5=> BootMode::Sd1BootMode,
        6=> BootMode::EmmcBootMode,
        7=> BootMode::UsbBootMode,
        14=> BootMode::Sd1LSBootMode,
        _=> BootMode::UndefinedBootMode
    }
}

// Sets the alt boot mode to mode and configures the register to boot from it
#[inline(always)]
pub fn crl_apb_set_user_alt_boot_mode( mode : BootMode ) -> () {
    let mut user_boot_mode_reg : u32;
    // Read out the current value
    unsafe {
        user_boot_mode_reg = crl_apb_reg_read(CRL_APB_BOOT_MODE_USER_REG_OFFSET);
    }
    // clear out the old alt boot mode
    user_boot_mode_reg = user_boot_mode_reg & !(CRL_APB_BOOT_MODE_USER_ALT_BOOT_MODE_MASK);
    // Set the new one as well as the use alt bit
    user_boot_mode_reg = user_boot_mode_reg | ( ( mode as u32 ) << CRL_APB_BOOT_MODE_USER_ALT_BOOT_MODE_OFF) | CRL_APB_BOOT_MODE_USER_USE_ALT_MASK;
    // Write it back in
    unsafe {
        crl_apb_reg_write(CRL_APB_BOOT_MODE_USER_REG_OFFSET, user_boot_mode_reg);
    }
}
