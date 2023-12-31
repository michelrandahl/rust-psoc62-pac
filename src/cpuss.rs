#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    identity: IDENTITY,
    cm4_status: CM4_STATUS,
    cm4_clock_ctl: CM4_CLOCK_CTL,
    cm4_ctl: CM4_CTL,
    _reserved4: [u8; 0xf0],
    cm4_int0_status: CM4_INT0_STATUS,
    cm4_int1_status: CM4_INT1_STATUS,
    cm4_int2_status: CM4_INT2_STATUS,
    cm4_int3_status: CM4_INT3_STATUS,
    cm4_int4_status: CM4_INT4_STATUS,
    cm4_int5_status: CM4_INT5_STATUS,
    cm4_int6_status: CM4_INT6_STATUS,
    cm4_int7_status: CM4_INT7_STATUS,
    _reserved12: [u8; 0xe0],
    cm4_vector_table_base: CM4_VECTOR_TABLE_BASE,
    _reserved13: [u8; 0x3c],
    cm4_nmi_ctl: [CM4_NMI_CTL; 4],
    _reserved14: [u8; 0xb0],
    udb_pwr_ctl: UDB_PWR_CTL,
    udb_pwr_delay_ctl: UDB_PWR_DELAY_CTL,
    _reserved16: [u8; 0x0cf8],
    cm0_ctl: CM0_CTL,
    cm0_status: CM0_STATUS,
    cm0_clock_ctl: CM0_CLOCK_CTL,
    _reserved19: [u8; 0xf4],
    cm0_int0_status: CM0_INT0_STATUS,
    cm0_int1_status: CM0_INT1_STATUS,
    cm0_int2_status: CM0_INT2_STATUS,
    cm0_int3_status: CM0_INT3_STATUS,
    cm0_int4_status: CM0_INT4_STATUS,
    cm0_int5_status: CM0_INT5_STATUS,
    cm0_int6_status: CM0_INT6_STATUS,
    cm0_int7_status: CM0_INT7_STATUS,
    cm0_vector_table_base: CM0_VECTOR_TABLE_BASE,
    _reserved28: [u8; 0x1c],
    cm0_nmi_ctl: [CM0_NMI_CTL; 4],
    _reserved29: [u8; 0xb0],
    cm4_pwr_ctl: CM4_PWR_CTL,
    cm4_pwr_delay_ctl: CM4_PWR_DELAY_CTL,
    _reserved31: [u8; 0xf8],
    ram0_ctl0: RAM0_CTL0,
    ram0_status: RAM0_STATUS,
    _reserved33: [u8; 0x38],
    ram0_pwr_macro_ctl: [RAM0_PWR_MACRO_CTL; 16],
    ram1_ctl0: RAM1_CTL0,
    ram1_status: RAM1_STATUS,
    ram1_pwr_ctl: RAM1_PWR_CTL,
    _reserved37: [u8; 0x14],
    ram2_ctl0: RAM2_CTL0,
    ram2_status: RAM2_STATUS,
    ram2_pwr_ctl: RAM2_PWR_CTL,
    _reserved40: [u8; 0x14],
    ram_pwr_delay_ctl: RAM_PWR_DELAY_CTL,
    rom_ctl: ROM_CTL,
    ecc_ctl: ECC_CTL,
    _reserved43: [u8; 0x34],
    product_id: PRODUCT_ID,
    _reserved44: [u8; 0x0c],
    dp_status: DP_STATUS,
    ap_ctl: AP_CTL,
    _reserved46: [u8; 0xe8],
    buff_ctl: BUFF_CTL,
    _reserved47: [u8; 0xfc],
    systick_ctl: SYSTICK_CTL,
    _reserved48: [u8; 0x0100],
    mbist_stat: MBIST_STAT,
    _reserved49: [u8; 0xf8],
    cal_sup_set: CAL_SUP_SET,
    cal_sup_clr: CAL_SUP_CLR,
    _reserved51: [u8; 0x07f8],
    cm0_pc_ctl: CM0_PC_CTL,
    _reserved52: [u8; 0x3c],
    cm0_pc0_handler: CM0_PC0_HANDLER,
    cm0_pc1_handler: CM0_PC1_HANDLER,
    cm0_pc2_handler: CM0_PC2_HANDLER,
    cm0_pc3_handler: CM0_PC3_HANDLER,
    _reserved56: [u8; 0x74],
    protection: PROTECTION,
    _reserved57: [u8; 0x38],
    trim_rom_ctl: TRIM_ROM_CTL,
    trim_ram_ctl: TRIM_RAM_CTL,
    _reserved59: [u8; 0x5ef8],
    cm0_system_int_ctl: [CM0_SYSTEM_INT_CTL; 1023],
    _reserved60: [u8; 0x1004],
    cm4_system_int_ctl: [CM4_SYSTEM_INT_CTL; 1023],
}
impl RegisterBlock {
    #[doc = "0x00 - Identity"]
    #[inline(always)]
    pub const fn identity(&self) -> &IDENTITY {
        &self.identity
    }
    #[doc = "0x04 - CM4 status"]
    #[inline(always)]
    pub const fn cm4_status(&self) -> &CM4_STATUS {
        &self.cm4_status
    }
    #[doc = "0x08 - CM4 clock control"]
    #[inline(always)]
    pub const fn cm4_clock_ctl(&self) -> &CM4_CLOCK_CTL {
        &self.cm4_clock_ctl
    }
    #[doc = "0x0c - CM4 control"]
    #[inline(always)]
    pub const fn cm4_ctl(&self) -> &CM4_CTL {
        &self.cm4_ctl
    }
    #[doc = "0x100 - CM4 interrupt 0 status"]
    #[inline(always)]
    pub const fn cm4_int0_status(&self) -> &CM4_INT0_STATUS {
        &self.cm4_int0_status
    }
    #[doc = "0x104 - CM4 interrupt 1 status"]
    #[inline(always)]
    pub const fn cm4_int1_status(&self) -> &CM4_INT1_STATUS {
        &self.cm4_int1_status
    }
    #[doc = "0x108 - CM4 interrupt 2 status"]
    #[inline(always)]
    pub const fn cm4_int2_status(&self) -> &CM4_INT2_STATUS {
        &self.cm4_int2_status
    }
    #[doc = "0x10c - CM4 interrupt 3 status"]
    #[inline(always)]
    pub const fn cm4_int3_status(&self) -> &CM4_INT3_STATUS {
        &self.cm4_int3_status
    }
    #[doc = "0x110 - CM4 interrupt 4 status"]
    #[inline(always)]
    pub const fn cm4_int4_status(&self) -> &CM4_INT4_STATUS {
        &self.cm4_int4_status
    }
    #[doc = "0x114 - CM4 interrupt 5 status"]
    #[inline(always)]
    pub const fn cm4_int5_status(&self) -> &CM4_INT5_STATUS {
        &self.cm4_int5_status
    }
    #[doc = "0x118 - CM4 interrupt 6 status"]
    #[inline(always)]
    pub const fn cm4_int6_status(&self) -> &CM4_INT6_STATUS {
        &self.cm4_int6_status
    }
    #[doc = "0x11c - CM4 interrupt 7 status"]
    #[inline(always)]
    pub const fn cm4_int7_status(&self) -> &CM4_INT7_STATUS {
        &self.cm4_int7_status
    }
    #[doc = "0x200 - CM4 vector table base"]
    #[inline(always)]
    pub const fn cm4_vector_table_base(&self) -> &CM4_VECTOR_TABLE_BASE {
        &self.cm4_vector_table_base
    }
    #[doc = "0x240..0x250 - CM4 NMI control"]
    #[inline(always)]
    pub const fn cm4_nmi_ctl(&self, n: usize) -> &CM4_NMI_CTL {
        &self.cm4_nmi_ctl[n]
    }
    #[doc = "0x300 - UDB power control"]
    #[inline(always)]
    pub const fn udb_pwr_ctl(&self) -> &UDB_PWR_CTL {
        &self.udb_pwr_ctl
    }
    #[doc = "0x304 - UDB power control"]
    #[inline(always)]
    pub const fn udb_pwr_delay_ctl(&self) -> &UDB_PWR_DELAY_CTL {
        &self.udb_pwr_delay_ctl
    }
    #[doc = "0x1000 - CM0+ control"]
    #[inline(always)]
    pub const fn cm0_ctl(&self) -> &CM0_CTL {
        &self.cm0_ctl
    }
    #[doc = "0x1004 - CM0+ status"]
    #[inline(always)]
    pub const fn cm0_status(&self) -> &CM0_STATUS {
        &self.cm0_status
    }
    #[doc = "0x1008 - CM0+ clock control"]
    #[inline(always)]
    pub const fn cm0_clock_ctl(&self) -> &CM0_CLOCK_CTL {
        &self.cm0_clock_ctl
    }
    #[doc = "0x1100 - CM0+ interrupt 0 status"]
    #[inline(always)]
    pub const fn cm0_int0_status(&self) -> &CM0_INT0_STATUS {
        &self.cm0_int0_status
    }
    #[doc = "0x1104 - CM0+ interrupt 1 status"]
    #[inline(always)]
    pub const fn cm0_int1_status(&self) -> &CM0_INT1_STATUS {
        &self.cm0_int1_status
    }
    #[doc = "0x1108 - CM0+ interrupt 2 status"]
    #[inline(always)]
    pub const fn cm0_int2_status(&self) -> &CM0_INT2_STATUS {
        &self.cm0_int2_status
    }
    #[doc = "0x110c - CM0+ interrupt 3 status"]
    #[inline(always)]
    pub const fn cm0_int3_status(&self) -> &CM0_INT3_STATUS {
        &self.cm0_int3_status
    }
    #[doc = "0x1110 - CM0+ interrupt 4 status"]
    #[inline(always)]
    pub const fn cm0_int4_status(&self) -> &CM0_INT4_STATUS {
        &self.cm0_int4_status
    }
    #[doc = "0x1114 - CM0+ interrupt 5 status"]
    #[inline(always)]
    pub const fn cm0_int5_status(&self) -> &CM0_INT5_STATUS {
        &self.cm0_int5_status
    }
    #[doc = "0x1118 - CM0+ interrupt 6 status"]
    #[inline(always)]
    pub const fn cm0_int6_status(&self) -> &CM0_INT6_STATUS {
        &self.cm0_int6_status
    }
    #[doc = "0x111c - CM0+ interrupt 7 status"]
    #[inline(always)]
    pub const fn cm0_int7_status(&self) -> &CM0_INT7_STATUS {
        &self.cm0_int7_status
    }
    #[doc = "0x1120 - CM0+ vector table base"]
    #[inline(always)]
    pub const fn cm0_vector_table_base(&self) -> &CM0_VECTOR_TABLE_BASE {
        &self.cm0_vector_table_base
    }
    #[doc = "0x1140..0x1150 - CM0+ NMI control"]
    #[inline(always)]
    pub const fn cm0_nmi_ctl(&self, n: usize) -> &CM0_NMI_CTL {
        &self.cm0_nmi_ctl[n]
    }
    #[doc = "0x1200 - CM4 power control"]
    #[inline(always)]
    pub const fn cm4_pwr_ctl(&self) -> &CM4_PWR_CTL {
        &self.cm4_pwr_ctl
    }
    #[doc = "0x1204 - CM4 power control"]
    #[inline(always)]
    pub const fn cm4_pwr_delay_ctl(&self) -> &CM4_PWR_DELAY_CTL {
        &self.cm4_pwr_delay_ctl
    }
    #[doc = "0x1300 - RAM 0 control"]
    #[inline(always)]
    pub const fn ram0_ctl0(&self) -> &RAM0_CTL0 {
        &self.ram0_ctl0
    }
    #[doc = "0x1304 - RAM 0 status"]
    #[inline(always)]
    pub const fn ram0_status(&self) -> &RAM0_STATUS {
        &self.ram0_status
    }
    #[doc = "0x1340..0x1380 - RAM 0 power control"]
    #[inline(always)]
    pub const fn ram0_pwr_macro_ctl(&self, n: usize) -> &RAM0_PWR_MACRO_CTL {
        &self.ram0_pwr_macro_ctl[n]
    }
    #[doc = "0x1380 - RAM 1 control"]
    #[inline(always)]
    pub const fn ram1_ctl0(&self) -> &RAM1_CTL0 {
        &self.ram1_ctl0
    }
    #[doc = "0x1384 - RAM 1 status"]
    #[inline(always)]
    pub const fn ram1_status(&self) -> &RAM1_STATUS {
        &self.ram1_status
    }
    #[doc = "0x1388 - RAM 1 power control"]
    #[inline(always)]
    pub const fn ram1_pwr_ctl(&self) -> &RAM1_PWR_CTL {
        &self.ram1_pwr_ctl
    }
    #[doc = "0x13a0 - RAM 2 control"]
    #[inline(always)]
    pub const fn ram2_ctl0(&self) -> &RAM2_CTL0 {
        &self.ram2_ctl0
    }
    #[doc = "0x13a4 - RAM 2 status"]
    #[inline(always)]
    pub const fn ram2_status(&self) -> &RAM2_STATUS {
        &self.ram2_status
    }
    #[doc = "0x13a8 - RAM 2 power control"]
    #[inline(always)]
    pub const fn ram2_pwr_ctl(&self) -> &RAM2_PWR_CTL {
        &self.ram2_pwr_ctl
    }
    #[doc = "0x13c0 - Power up delay used for all SRAM power domains"]
    #[inline(always)]
    pub const fn ram_pwr_delay_ctl(&self) -> &RAM_PWR_DELAY_CTL {
        &self.ram_pwr_delay_ctl
    }
    #[doc = "0x13c4 - ROM control"]
    #[inline(always)]
    pub const fn rom_ctl(&self) -> &ROM_CTL {
        &self.rom_ctl
    }
    #[doc = "0x13c8 - ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(&self) -> &ECC_CTL {
        &self.ecc_ctl
    }
    #[doc = "0x1400 - Product identifier and version (same as CoreSight RomTables)"]
    #[inline(always)]
    pub const fn product_id(&self) -> &PRODUCT_ID {
        &self.product_id
    }
    #[doc = "0x1410 - Debug port status"]
    #[inline(always)]
    pub const fn dp_status(&self) -> &DP_STATUS {
        &self.dp_status
    }
    #[doc = "0x1414 - Access port control"]
    #[inline(always)]
    pub const fn ap_ctl(&self) -> &AP_CTL {
        &self.ap_ctl
    }
    #[doc = "0x1500 - Buffer control"]
    #[inline(always)]
    pub const fn buff_ctl(&self) -> &BUFF_CTL {
        &self.buff_ctl
    }
    #[doc = "0x1600 - SysTick timer control"]
    #[inline(always)]
    pub const fn systick_ctl(&self) -> &SYSTICK_CTL {
        &self.systick_ctl
    }
    #[doc = "0x1704 - Memory BIST status"]
    #[inline(always)]
    pub const fn mbist_stat(&self) -> &MBIST_STAT {
        &self.mbist_stat
    }
    #[doc = "0x1800 - Calibration support set and read"]
    #[inline(always)]
    pub const fn cal_sup_set(&self) -> &CAL_SUP_SET {
        &self.cal_sup_set
    }
    #[doc = "0x1804 - Calibration support clear and reset"]
    #[inline(always)]
    pub const fn cal_sup_clr(&self) -> &CAL_SUP_CLR {
        &self.cal_sup_clr
    }
    #[doc = "0x2000 - CM0+ protection context control"]
    #[inline(always)]
    pub const fn cm0_pc_ctl(&self) -> &CM0_PC_CTL {
        &self.cm0_pc_ctl
    }
    #[doc = "0x2040 - CM0+ protection context 0 handler"]
    #[inline(always)]
    pub const fn cm0_pc0_handler(&self) -> &CM0_PC0_HANDLER {
        &self.cm0_pc0_handler
    }
    #[doc = "0x2044 - CM0+ protection context 1 handler"]
    #[inline(always)]
    pub const fn cm0_pc1_handler(&self) -> &CM0_PC1_HANDLER {
        &self.cm0_pc1_handler
    }
    #[doc = "0x2048 - CM0+ protection context 2 handler"]
    #[inline(always)]
    pub const fn cm0_pc2_handler(&self) -> &CM0_PC2_HANDLER {
        &self.cm0_pc2_handler
    }
    #[doc = "0x204c - CM0+ protection context 3 handler"]
    #[inline(always)]
    pub const fn cm0_pc3_handler(&self) -> &CM0_PC3_HANDLER {
        &self.cm0_pc3_handler
    }
    #[doc = "0x20c4 - Protection status"]
    #[inline(always)]
    pub const fn protection(&self) -> &PROTECTION {
        &self.protection
    }
    #[doc = "0x2100 - ROM trim control"]
    #[inline(always)]
    pub const fn trim_rom_ctl(&self) -> &TRIM_ROM_CTL {
        &self.trim_rom_ctl
    }
    #[doc = "0x2104 - RAM trim control"]
    #[inline(always)]
    pub const fn trim_ram_ctl(&self) -> &TRIM_RAM_CTL {
        &self.trim_ram_ctl
    }
    #[doc = "0x8000..0x8ffc - CM0+ system interrupt control"]
    #[inline(always)]
    pub const fn cm0_system_int_ctl(&self, n: usize) -> &CM0_SYSTEM_INT_CTL {
        &self.cm0_system_int_ctl[n]
    }
    #[doc = "0xa000..0xaffc - CM4 system interrupt control"]
    #[inline(always)]
    pub const fn cm4_system_int_ctl(&self, n: usize) -> &CM4_SYSTEM_INT_CTL {
        &self.cm4_system_int_ctl[n]
    }
}
#[doc = "IDENTITY (r) register accessor: Identity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`identity::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@identity`]
module"]
pub type IDENTITY = crate::Reg<identity::IDENTITY_SPEC>;
#[doc = "Identity"]
pub mod identity;
#[doc = "CM4_STATUS (r) register accessor: CM4 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_status`]
module"]
pub type CM4_STATUS = crate::Reg<cm4_status::CM4_STATUS_SPEC>;
#[doc = "CM4 status"]
pub mod cm4_status;
#[doc = "CM4_CLOCK_CTL (rw) register accessor: CM4 clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_clock_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_clock_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_clock_ctl`]
module"]
pub type CM4_CLOCK_CTL = crate::Reg<cm4_clock_ctl::CM4_CLOCK_CTL_SPEC>;
#[doc = "CM4 clock control"]
pub mod cm4_clock_ctl;
#[doc = "CM4_CTL (rw) register accessor: CM4 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ctl`]
module"]
pub type CM4_CTL = crate::Reg<cm4_ctl::CM4_CTL_SPEC>;
#[doc = "CM4 control"]
pub mod cm4_ctl;
#[doc = "CM4_INT0_STATUS (r) register accessor: CM4 interrupt 0 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_int0_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int0_status`]
module"]
pub type CM4_INT0_STATUS = crate::Reg<cm4_int0_status::CM4_INT0_STATUS_SPEC>;
#[doc = "CM4 interrupt 0 status"]
pub mod cm4_int0_status;
#[doc = "CM4_INT1_STATUS (r) register accessor: CM4 interrupt 1 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_int1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int1_status`]
module"]
pub type CM4_INT1_STATUS = crate::Reg<cm4_int1_status::CM4_INT1_STATUS_SPEC>;
#[doc = "CM4 interrupt 1 status"]
pub mod cm4_int1_status;
#[doc = "CM4_INT2_STATUS (r) register accessor: CM4 interrupt 2 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_int2_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int2_status`]
module"]
pub type CM4_INT2_STATUS = crate::Reg<cm4_int2_status::CM4_INT2_STATUS_SPEC>;
#[doc = "CM4 interrupt 2 status"]
pub mod cm4_int2_status;
#[doc = "CM4_INT3_STATUS (r) register accessor: CM4 interrupt 3 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_int3_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int3_status`]
module"]
pub type CM4_INT3_STATUS = crate::Reg<cm4_int3_status::CM4_INT3_STATUS_SPEC>;
#[doc = "CM4 interrupt 3 status"]
pub mod cm4_int3_status;
#[doc = "CM4_INT4_STATUS (r) register accessor: CM4 interrupt 4 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_int4_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int4_status`]
module"]
pub type CM4_INT4_STATUS = crate::Reg<cm4_int4_status::CM4_INT4_STATUS_SPEC>;
#[doc = "CM4 interrupt 4 status"]
pub mod cm4_int4_status;
#[doc = "CM4_INT5_STATUS (r) register accessor: CM4 interrupt 5 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_int5_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int5_status`]
module"]
pub type CM4_INT5_STATUS = crate::Reg<cm4_int5_status::CM4_INT5_STATUS_SPEC>;
#[doc = "CM4 interrupt 5 status"]
pub mod cm4_int5_status;
#[doc = "CM4_INT6_STATUS (r) register accessor: CM4 interrupt 6 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_int6_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int6_status`]
module"]
pub type CM4_INT6_STATUS = crate::Reg<cm4_int6_status::CM4_INT6_STATUS_SPEC>;
#[doc = "CM4 interrupt 6 status"]
pub mod cm4_int6_status;
#[doc = "CM4_INT7_STATUS (r) register accessor: CM4 interrupt 7 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_int7_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int7_status`]
module"]
pub type CM4_INT7_STATUS = crate::Reg<cm4_int7_status::CM4_INT7_STATUS_SPEC>;
#[doc = "CM4 interrupt 7 status"]
pub mod cm4_int7_status;
#[doc = "CM4_VECTOR_TABLE_BASE (rw) register accessor: CM4 vector table base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_vector_table_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_vector_table_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_vector_table_base`]
module"]
pub type CM4_VECTOR_TABLE_BASE = crate::Reg<cm4_vector_table_base::CM4_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM4 vector table base"]
pub mod cm4_vector_table_base;
#[doc = "CM4_NMI_CTL (rw) register accessor: CM4 NMI control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_nmi_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_nmi_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_nmi_ctl`]
module"]
pub type CM4_NMI_CTL = crate::Reg<cm4_nmi_ctl::CM4_NMI_CTL_SPEC>;
#[doc = "CM4 NMI control"]
pub mod cm4_nmi_ctl;
#[doc = "UDB_PWR_CTL (rw) register accessor: UDB power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udb_pwr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udb_pwr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udb_pwr_ctl`]
module"]
pub type UDB_PWR_CTL = crate::Reg<udb_pwr_ctl::UDB_PWR_CTL_SPEC>;
#[doc = "UDB power control"]
pub mod udb_pwr_ctl;
#[doc = "UDB_PWR_DELAY_CTL (rw) register accessor: UDB power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udb_pwr_delay_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udb_pwr_delay_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udb_pwr_delay_ctl`]
module"]
pub type UDB_PWR_DELAY_CTL = crate::Reg<udb_pwr_delay_ctl::UDB_PWR_DELAY_CTL_SPEC>;
#[doc = "UDB power control"]
pub mod udb_pwr_delay_ctl;
#[doc = "CM0_CTL (rw) register accessor: CM0+ control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ctl`]
module"]
pub type CM0_CTL = crate::Reg<cm0_ctl::CM0_CTL_SPEC>;
#[doc = "CM0+ control"]
pub mod cm0_ctl;
#[doc = "CM0_STATUS (r) register accessor: CM0+ status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_status`]
module"]
pub type CM0_STATUS = crate::Reg<cm0_status::CM0_STATUS_SPEC>;
#[doc = "CM0+ status"]
pub mod cm0_status;
#[doc = "CM0_CLOCK_CTL (rw) register accessor: CM0+ clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_clock_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_clock_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_clock_ctl`]
module"]
pub type CM0_CLOCK_CTL = crate::Reg<cm0_clock_ctl::CM0_CLOCK_CTL_SPEC>;
#[doc = "CM0+ clock control"]
pub mod cm0_clock_ctl;
#[doc = "CM0_INT0_STATUS (r) register accessor: CM0+ interrupt 0 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_int0_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int0_status`]
module"]
pub type CM0_INT0_STATUS = crate::Reg<cm0_int0_status::CM0_INT0_STATUS_SPEC>;
#[doc = "CM0+ interrupt 0 status"]
pub mod cm0_int0_status;
#[doc = "CM0_INT1_STATUS (r) register accessor: CM0+ interrupt 1 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_int1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int1_status`]
module"]
pub type CM0_INT1_STATUS = crate::Reg<cm0_int1_status::CM0_INT1_STATUS_SPEC>;
#[doc = "CM0+ interrupt 1 status"]
pub mod cm0_int1_status;
#[doc = "CM0_INT2_STATUS (r) register accessor: CM0+ interrupt 2 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_int2_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int2_status`]
module"]
pub type CM0_INT2_STATUS = crate::Reg<cm0_int2_status::CM0_INT2_STATUS_SPEC>;
#[doc = "CM0+ interrupt 2 status"]
pub mod cm0_int2_status;
#[doc = "CM0_INT3_STATUS (r) register accessor: CM0+ interrupt 3 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_int3_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int3_status`]
module"]
pub type CM0_INT3_STATUS = crate::Reg<cm0_int3_status::CM0_INT3_STATUS_SPEC>;
#[doc = "CM0+ interrupt 3 status"]
pub mod cm0_int3_status;
#[doc = "CM0_INT4_STATUS (r) register accessor: CM0+ interrupt 4 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_int4_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int4_status`]
module"]
pub type CM0_INT4_STATUS = crate::Reg<cm0_int4_status::CM0_INT4_STATUS_SPEC>;
#[doc = "CM0+ interrupt 4 status"]
pub mod cm0_int4_status;
#[doc = "CM0_INT5_STATUS (r) register accessor: CM0+ interrupt 5 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_int5_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int5_status`]
module"]
pub type CM0_INT5_STATUS = crate::Reg<cm0_int5_status::CM0_INT5_STATUS_SPEC>;
#[doc = "CM0+ interrupt 5 status"]
pub mod cm0_int5_status;
#[doc = "CM0_INT6_STATUS (r) register accessor: CM0+ interrupt 6 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_int6_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int6_status`]
module"]
pub type CM0_INT6_STATUS = crate::Reg<cm0_int6_status::CM0_INT6_STATUS_SPEC>;
#[doc = "CM0+ interrupt 6 status"]
pub mod cm0_int6_status;
#[doc = "CM0_INT7_STATUS (r) register accessor: CM0+ interrupt 7 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_int7_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int7_status`]
module"]
pub type CM0_INT7_STATUS = crate::Reg<cm0_int7_status::CM0_INT7_STATUS_SPEC>;
#[doc = "CM0+ interrupt 7 status"]
pub mod cm0_int7_status;
#[doc = "CM0_VECTOR_TABLE_BASE (rw) register accessor: CM0+ vector table base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_vector_table_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_vector_table_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_vector_table_base`]
module"]
pub type CM0_VECTOR_TABLE_BASE = crate::Reg<cm0_vector_table_base::CM0_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM0+ vector table base"]
pub mod cm0_vector_table_base;
#[doc = "CM0_NMI_CTL (rw) register accessor: CM0+ NMI control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_nmi_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_nmi_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_nmi_ctl`]
module"]
pub type CM0_NMI_CTL = crate::Reg<cm0_nmi_ctl::CM0_NMI_CTL_SPEC>;
#[doc = "CM0+ NMI control"]
pub mod cm0_nmi_ctl;
#[doc = "CM4_PWR_CTL (rw) register accessor: CM4 power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_pwr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_pwr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_pwr_ctl`]
module"]
pub type CM4_PWR_CTL = crate::Reg<cm4_pwr_ctl::CM4_PWR_CTL_SPEC>;
#[doc = "CM4 power control"]
pub mod cm4_pwr_ctl;
#[doc = "CM4_PWR_DELAY_CTL (rw) register accessor: CM4 power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_pwr_delay_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_pwr_delay_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_pwr_delay_ctl`]
module"]
pub type CM4_PWR_DELAY_CTL = crate::Reg<cm4_pwr_delay_ctl::CM4_PWR_DELAY_CTL_SPEC>;
#[doc = "CM4 power control"]
pub mod cm4_pwr_delay_ctl;
#[doc = "RAM0_CTL0 (rw) register accessor: RAM 0 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram0_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram0_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram0_ctl0`]
module"]
pub type RAM0_CTL0 = crate::Reg<ram0_ctl0::RAM0_CTL0_SPEC>;
#[doc = "RAM 0 control"]
pub mod ram0_ctl0;
#[doc = "RAM0_STATUS (r) register accessor: RAM 0 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram0_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram0_status`]
module"]
pub type RAM0_STATUS = crate::Reg<ram0_status::RAM0_STATUS_SPEC>;
#[doc = "RAM 0 status"]
pub mod ram0_status;
#[doc = "RAM0_PWR_MACRO_CTL (rw) register accessor: RAM 0 power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram0_pwr_macro_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram0_pwr_macro_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram0_pwr_macro_ctl`]
module"]
pub type RAM0_PWR_MACRO_CTL = crate::Reg<ram0_pwr_macro_ctl::RAM0_PWR_MACRO_CTL_SPEC>;
#[doc = "RAM 0 power control"]
pub mod ram0_pwr_macro_ctl;
#[doc = "RAM1_CTL0 (rw) register accessor: RAM 1 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram1_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram1_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1_ctl0`]
module"]
pub type RAM1_CTL0 = crate::Reg<ram1_ctl0::RAM1_CTL0_SPEC>;
#[doc = "RAM 1 control"]
pub mod ram1_ctl0;
#[doc = "RAM1_STATUS (r) register accessor: RAM 1 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1_status`]
module"]
pub type RAM1_STATUS = crate::Reg<ram1_status::RAM1_STATUS_SPEC>;
#[doc = "RAM 1 status"]
pub mod ram1_status;
#[doc = "RAM1_PWR_CTL (rw) register accessor: RAM 1 power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram1_pwr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram1_pwr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1_pwr_ctl`]
module"]
pub type RAM1_PWR_CTL = crate::Reg<ram1_pwr_ctl::RAM1_PWR_CTL_SPEC>;
#[doc = "RAM 1 power control"]
pub mod ram1_pwr_ctl;
#[doc = "RAM2_CTL0 (rw) register accessor: RAM 2 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2_ctl0`]
module"]
pub type RAM2_CTL0 = crate::Reg<ram2_ctl0::RAM2_CTL0_SPEC>;
#[doc = "RAM 2 control"]
pub mod ram2_ctl0;
#[doc = "RAM2_STATUS (r) register accessor: RAM 2 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2_status`]
module"]
pub type RAM2_STATUS = crate::Reg<ram2_status::RAM2_STATUS_SPEC>;
#[doc = "RAM 2 status"]
pub mod ram2_status;
#[doc = "RAM2_PWR_CTL (rw) register accessor: RAM 2 power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2_pwr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2_pwr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2_pwr_ctl`]
module"]
pub type RAM2_PWR_CTL = crate::Reg<ram2_pwr_ctl::RAM2_PWR_CTL_SPEC>;
#[doc = "RAM 2 power control"]
pub mod ram2_pwr_ctl;
#[doc = "RAM_PWR_DELAY_CTL (rw) register accessor: Power up delay used for all SRAM power domains\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_pwr_delay_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_pwr_delay_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_pwr_delay_ctl`]
module"]
pub type RAM_PWR_DELAY_CTL = crate::Reg<ram_pwr_delay_ctl::RAM_PWR_DELAY_CTL_SPEC>;
#[doc = "Power up delay used for all SRAM power domains"]
pub mod ram_pwr_delay_ctl;
#[doc = "ROM_CTL (rw) register accessor: ROM control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_ctl`]
module"]
pub type ROM_CTL = crate::Reg<rom_ctl::ROM_CTL_SPEC>;
#[doc = "ROM control"]
pub mod rom_ctl;
#[doc = "ECC_CTL (rw) register accessor: ECC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctl`]
module"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "PRODUCT_ID (r) register accessor: Product identifier and version (same as CoreSight RomTables)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@product_id`]
module"]
pub type PRODUCT_ID = crate::Reg<product_id::PRODUCT_ID_SPEC>;
#[doc = "Product identifier and version (same as CoreSight RomTables)"]
pub mod product_id;
#[doc = "DP_STATUS (r) register accessor: Debug port status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_status`]
module"]
pub type DP_STATUS = crate::Reg<dp_status::DP_STATUS_SPEC>;
#[doc = "Debug port status"]
pub mod dp_status;
#[doc = "AP_CTL (rw) register accessor: Access port control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ap_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ap_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ap_ctl`]
module"]
pub type AP_CTL = crate::Reg<ap_ctl::AP_CTL_SPEC>;
#[doc = "Access port control"]
pub mod ap_ctl;
#[doc = "BUFF_CTL (rw) register accessor: Buffer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buff_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buff_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff_ctl`]
module"]
pub type BUFF_CTL = crate::Reg<buff_ctl::BUFF_CTL_SPEC>;
#[doc = "Buffer control"]
pub mod buff_ctl;
#[doc = "SYSTICK_CTL (rw) register accessor: SysTick timer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systick_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systick_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systick_ctl`]
module"]
pub type SYSTICK_CTL = crate::Reg<systick_ctl::SYSTICK_CTL_SPEC>;
#[doc = "SysTick timer control"]
pub mod systick_ctl;
#[doc = "MBIST_STAT (r) register accessor: Memory BIST status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbist_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbist_stat`]
module"]
pub type MBIST_STAT = crate::Reg<mbist_stat::MBIST_STAT_SPEC>;
#[doc = "Memory BIST status"]
pub mod mbist_stat;
#[doc = "CAL_SUP_SET (rw) register accessor: Calibration support set and read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_sup_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_sup_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_sup_set`]
module"]
pub type CAL_SUP_SET = crate::Reg<cal_sup_set::CAL_SUP_SET_SPEC>;
#[doc = "Calibration support set and read"]
pub mod cal_sup_set;
#[doc = "CAL_SUP_CLR (rw) register accessor: Calibration support clear and reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_sup_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_sup_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_sup_clr`]
module"]
pub type CAL_SUP_CLR = crate::Reg<cal_sup_clr::CAL_SUP_CLR_SPEC>;
#[doc = "Calibration support clear and reset"]
pub mod cal_sup_clr;
#[doc = "CM0_PC_CTL (rw) register accessor: CM0+ protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_pc_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_pc_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_pc_ctl`]
module"]
pub type CM0_PC_CTL = crate::Reg<cm0_pc_ctl::CM0_PC_CTL_SPEC>;
#[doc = "CM0+ protection context control"]
pub mod cm0_pc_ctl;
#[doc = "CM0_PC0_HANDLER (rw) register accessor: CM0+ protection context 0 handler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_pc0_handler::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_pc0_handler::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_pc0_handler`]
module"]
pub type CM0_PC0_HANDLER = crate::Reg<cm0_pc0_handler::CM0_PC0_HANDLER_SPEC>;
#[doc = "CM0+ protection context 0 handler"]
pub mod cm0_pc0_handler;
#[doc = "CM0_PC1_HANDLER (rw) register accessor: CM0+ protection context 1 handler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_pc1_handler::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_pc1_handler::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_pc1_handler`]
module"]
pub type CM0_PC1_HANDLER = crate::Reg<cm0_pc1_handler::CM0_PC1_HANDLER_SPEC>;
#[doc = "CM0+ protection context 1 handler"]
pub mod cm0_pc1_handler;
#[doc = "CM0_PC2_HANDLER (rw) register accessor: CM0+ protection context 2 handler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_pc2_handler::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_pc2_handler::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_pc2_handler`]
module"]
pub type CM0_PC2_HANDLER = crate::Reg<cm0_pc2_handler::CM0_PC2_HANDLER_SPEC>;
#[doc = "CM0+ protection context 2 handler"]
pub mod cm0_pc2_handler;
#[doc = "CM0_PC3_HANDLER (rw) register accessor: CM0+ protection context 3 handler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_pc3_handler::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_pc3_handler::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_pc3_handler`]
module"]
pub type CM0_PC3_HANDLER = crate::Reg<cm0_pc3_handler::CM0_PC3_HANDLER_SPEC>;
#[doc = "CM0+ protection context 3 handler"]
pub mod cm0_pc3_handler;
#[doc = "PROTECTION (rw) register accessor: Protection status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`protection::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`protection::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@protection`]
module"]
pub type PROTECTION = crate::Reg<protection::PROTECTION_SPEC>;
#[doc = "Protection status"]
pub mod protection;
#[doc = "TRIM_ROM_CTL (rw) register accessor: ROM trim control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim_rom_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim_rom_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim_rom_ctl`]
module"]
pub type TRIM_ROM_CTL = crate::Reg<trim_rom_ctl::TRIM_ROM_CTL_SPEC>;
#[doc = "ROM trim control"]
pub mod trim_rom_ctl;
#[doc = "TRIM_RAM_CTL (rw) register accessor: RAM trim control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim_ram_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim_ram_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim_ram_ctl`]
module"]
pub type TRIM_RAM_CTL = crate::Reg<trim_ram_ctl::TRIM_RAM_CTL_SPEC>;
#[doc = "RAM trim control"]
pub mod trim_ram_ctl;
#[doc = "CM0_SYSTEM_INT_CTL (rw) register accessor: CM0+ system interrupt control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_system_int_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_system_int_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_system_int_ctl`]
module"]
pub type CM0_SYSTEM_INT_CTL = crate::Reg<cm0_system_int_ctl::CM0_SYSTEM_INT_CTL_SPEC>;
#[doc = "CM0+ system interrupt control"]
pub mod cm0_system_int_ctl;
#[doc = "CM4_SYSTEM_INT_CTL (rw) register accessor: CM4 system interrupt control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_system_int_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_system_int_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_system_int_ctl`]
module"]
pub type CM4_SYSTEM_INT_CTL = crate::Reg<cm4_system_int_ctl::CM4_SYSTEM_INT_CTL_SPEC>;
#[doc = "CM4 system interrupt control"]
pub mod cm4_system_int_ctl;
