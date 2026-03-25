// CRL_APB
// XX XX LPD   XX XX XX
// Low Power Domain (LPD) clock and reset control

// CRL APB Config Register Baseaddress
const CRL_APB_REG_BASEADDRESS                : u32 = 0xFF5E0000;

// CRL APB Register Offsets
pub const CRL_APB_ERR_CTRL_REG_OFFSET        : u32 = 0x00000000;
pub const CRL_APB_IR_STATUS_REG_OFFSET       : u32 = 0x00000004;
pub const CRL_APB_IR_MASK_REG_OFFSET         : u32 = 0x00000008;
pub const CRL_APB_IR_ENABLE_REG_OFFSET       : u32 = 0x0000000C;
pub const CRL_APB_IR_DISABLE_REG_OFFSET      : u32 = 0x00000010;

pub const CRL_APB_CRL_WRPROT_REG_OFFSET      : u32 = 0x0000001C;
pub const CRL_APB_IOPLL_CTRL_REG_OFFSET      : u32 = 0x00000020;