#[doc = r"Register block"]
#[repr(C)]
pub struct USBHOST {
    host_ctl0: HOST_CTL0,
    _reserved1: [u8; 0x0c],
    host_ctl1: HOST_CTL1,
    _reserved2: [u8; 0xec],
    host_ctl2: HOST_CTL2,
    host_err: HOST_ERR,
    host_status: HOST_STATUS,
    host_fcomp: HOST_FCOMP,
    host_rtimer: HOST_RTIMER,
    host_addr: HOST_ADDR,
    host_eof: HOST_EOF,
    host_frame: HOST_FRAME,
    host_token: HOST_TOKEN,
    _reserved11: [u8; 0x02dc],
    host_ep1_ctl: HOST_EP1_CTL,
    host_ep1_status: HOST_EP1_STATUS,
    host_ep1_rw1_dr: HOST_EP1_RW1_DR,
    host_ep1_rw2_dr: HOST_EP1_RW2_DR,
    _reserved15: [u8; 0xf0],
    host_ep2_ctl: HOST_EP2_CTL,
    host_ep2_status: HOST_EP2_STATUS,
    host_ep2_rw1_dr: HOST_EP2_RW1_DR,
    host_ep2_rw2_dr: HOST_EP2_RW2_DR,
    _reserved19: [u8; 0x02f0],
    host_lvl1_sel: HOST_LVL1_SEL,
    host_lvl2_sel: HOST_LVL2_SEL,
    _reserved21: [u8; 0xf8],
    intr_usbhost_cause_hi: INTR_USBHOST_CAUSE_HI,
    intr_usbhost_cause_med: INTR_USBHOST_CAUSE_MED,
    intr_usbhost_cause_lo: INTR_USBHOST_CAUSE_LO,
    _reserved24: [u8; 0x14],
    intr_host_ep_cause_hi: INTR_HOST_EP_CAUSE_HI,
    intr_host_ep_cause_med: INTR_HOST_EP_CAUSE_MED,
    intr_host_ep_cause_lo: INTR_HOST_EP_CAUSE_LO,
    _reserved27: [u8; 0x14],
    intr_usbhost: INTR_USBHOST,
    intr_usbhost_set: INTR_USBHOST_SET,
    intr_usbhost_mask: INTR_USBHOST_MASK,
    intr_usbhost_masked: INTR_USBHOST_MASKED,
    _reserved31: [u8; 0xb0],
    intr_host_ep: INTR_HOST_EP,
    intr_host_ep_set: INTR_HOST_EP_SET,
    intr_host_ep_mask: INTR_HOST_EP_MASK,
    intr_host_ep_masked: INTR_HOST_EP_MASKED,
    _reserved35: [u8; 0xf0],
    host_dma_enbl: HOST_DMA_ENBL,
    _reserved36: [u8; 0x1c],
    host_ep1_blk: HOST_EP1_BLK,
    _reserved37: [u8; 0x0c],
    host_ep2_blk: HOST_EP2_BLK,
}
impl USBHOST {
    #[doc = "0x00 - Host Control 0 Register."]
    #[inline(always)]
    pub const fn host_ctl0(&self) -> &HOST_CTL0 {
        &self.host_ctl0
    }
    #[doc = "0x10 - Host Control 1 Register."]
    #[inline(always)]
    pub const fn host_ctl1(&self) -> &HOST_CTL1 {
        &self.host_ctl1
    }
    #[doc = "0x100 - Host Control 2 Register."]
    #[inline(always)]
    pub const fn host_ctl2(&self) -> &HOST_CTL2 {
        &self.host_ctl2
    }
    #[doc = "0x104 - Host Error Status Register."]
    #[inline(always)]
    pub const fn host_err(&self) -> &HOST_ERR {
        &self.host_err
    }
    #[doc = "0x108 - Host Status Register."]
    #[inline(always)]
    pub const fn host_status(&self) -> &HOST_STATUS {
        &self.host_status
    }
    #[doc = "0x10c - Host SOF Interrupt Frame Compare Register"]
    #[inline(always)]
    pub const fn host_fcomp(&self) -> &HOST_FCOMP {
        &self.host_fcomp
    }
    #[doc = "0x110 - Host Retry Timer Setup Register"]
    #[inline(always)]
    pub const fn host_rtimer(&self) -> &HOST_RTIMER {
        &self.host_rtimer
    }
    #[doc = "0x114 - Host Address Register"]
    #[inline(always)]
    pub const fn host_addr(&self) -> &HOST_ADDR {
        &self.host_addr
    }
    #[doc = "0x118 - Host EOF Setup Register"]
    #[inline(always)]
    pub const fn host_eof(&self) -> &HOST_EOF {
        &self.host_eof
    }
    #[doc = "0x11c - Host Frame Setup Register"]
    #[inline(always)]
    pub const fn host_frame(&self) -> &HOST_FRAME {
        &self.host_frame
    }
    #[doc = "0x120 - Host Token Endpoint Register"]
    #[inline(always)]
    pub const fn host_token(&self) -> &HOST_TOKEN {
        &self.host_token
    }
    #[doc = "0x400 - Host Endpoint 1 Control Register"]
    #[inline(always)]
    pub const fn host_ep1_ctl(&self) -> &HOST_EP1_CTL {
        &self.host_ep1_ctl
    }
    #[doc = "0x404 - Host Endpoint 1 Status Register"]
    #[inline(always)]
    pub const fn host_ep1_status(&self) -> &HOST_EP1_STATUS {
        &self.host_ep1_status
    }
    #[doc = "0x408 - Host Endpoint 1 Data 1-Byte Register"]
    #[inline(always)]
    pub const fn host_ep1_rw1_dr(&self) -> &HOST_EP1_RW1_DR {
        &self.host_ep1_rw1_dr
    }
    #[doc = "0x40c - Host Endpoint 1 Data 2-Byte Register"]
    #[inline(always)]
    pub const fn host_ep1_rw2_dr(&self) -> &HOST_EP1_RW2_DR {
        &self.host_ep1_rw2_dr
    }
    #[doc = "0x500 - Host Endpoint 2 Control Register"]
    #[inline(always)]
    pub const fn host_ep2_ctl(&self) -> &HOST_EP2_CTL {
        &self.host_ep2_ctl
    }
    #[doc = "0x504 - Host Endpoint 2 Status Register"]
    #[inline(always)]
    pub const fn host_ep2_status(&self) -> &HOST_EP2_STATUS {
        &self.host_ep2_status
    }
    #[doc = "0x508 - Host Endpoint 2 Data 1-Byte Register"]
    #[inline(always)]
    pub const fn host_ep2_rw1_dr(&self) -> &HOST_EP2_RW1_DR {
        &self.host_ep2_rw1_dr
    }
    #[doc = "0x50c - Host Endpoint 2 Data 2-Byte Register"]
    #[inline(always)]
    pub const fn host_ep2_rw2_dr(&self) -> &HOST_EP2_RW2_DR {
        &self.host_ep2_rw2_dr
    }
    #[doc = "0x800 - Host Interrupt Level 1 Selection Register"]
    #[inline(always)]
    pub const fn host_lvl1_sel(&self) -> &HOST_LVL1_SEL {
        &self.host_lvl1_sel
    }
    #[doc = "0x804 - Host Interrupt Level 2 Selection Register"]
    #[inline(always)]
    pub const fn host_lvl2_sel(&self) -> &HOST_LVL2_SEL {
        &self.host_lvl2_sel
    }
    #[doc = "0x900 - Interrupt USB Host Cause High Register"]
    #[inline(always)]
    pub const fn intr_usbhost_cause_hi(&self) -> &INTR_USBHOST_CAUSE_HI {
        &self.intr_usbhost_cause_hi
    }
    #[doc = "0x904 - Interrupt USB Host Cause Medium Register"]
    #[inline(always)]
    pub const fn intr_usbhost_cause_med(&self) -> &INTR_USBHOST_CAUSE_MED {
        &self.intr_usbhost_cause_med
    }
    #[doc = "0x908 - Interrupt USB Host Cause Low Register"]
    #[inline(always)]
    pub const fn intr_usbhost_cause_lo(&self) -> &INTR_USBHOST_CAUSE_LO {
        &self.intr_usbhost_cause_lo
    }
    #[doc = "0x920 - Interrupt USB Host Endpoint Cause High Register"]
    #[inline(always)]
    pub const fn intr_host_ep_cause_hi(&self) -> &INTR_HOST_EP_CAUSE_HI {
        &self.intr_host_ep_cause_hi
    }
    #[doc = "0x924 - Interrupt USB Host Endpoint Cause Medium Register"]
    #[inline(always)]
    pub const fn intr_host_ep_cause_med(&self) -> &INTR_HOST_EP_CAUSE_MED {
        &self.intr_host_ep_cause_med
    }
    #[doc = "0x928 - Interrupt USB Host Endpoint Cause Low Register"]
    #[inline(always)]
    pub const fn intr_host_ep_cause_lo(&self) -> &INTR_HOST_EP_CAUSE_LO {
        &self.intr_host_ep_cause_lo
    }
    #[doc = "0x940 - Interrupt USB Host Register"]
    #[inline(always)]
    pub const fn intr_usbhost(&self) -> &INTR_USBHOST {
        &self.intr_usbhost
    }
    #[doc = "0x944 - Interrupt USB Host Set Register"]
    #[inline(always)]
    pub const fn intr_usbhost_set(&self) -> &INTR_USBHOST_SET {
        &self.intr_usbhost_set
    }
    #[doc = "0x948 - Interrupt USB Host Mask Register"]
    #[inline(always)]
    pub const fn intr_usbhost_mask(&self) -> &INTR_USBHOST_MASK {
        &self.intr_usbhost_mask
    }
    #[doc = "0x94c - Interrupt USB Host Masked Register"]
    #[inline(always)]
    pub const fn intr_usbhost_masked(&self) -> &INTR_USBHOST_MASKED {
        &self.intr_usbhost_masked
    }
    #[doc = "0xa00 - Interrupt USB Host Endpoint Register"]
    #[inline(always)]
    pub const fn intr_host_ep(&self) -> &INTR_HOST_EP {
        &self.intr_host_ep
    }
    #[doc = "0xa04 - Interrupt USB Host Endpoint Set Register"]
    #[inline(always)]
    pub const fn intr_host_ep_set(&self) -> &INTR_HOST_EP_SET {
        &self.intr_host_ep_set
    }
    #[doc = "0xa08 - Interrupt USB Host Endpoint Mask Register"]
    #[inline(always)]
    pub const fn intr_host_ep_mask(&self) -> &INTR_HOST_EP_MASK {
        &self.intr_host_ep_mask
    }
    #[doc = "0xa0c - Interrupt USB Host Endpoint Masked Register"]
    #[inline(always)]
    pub const fn intr_host_ep_masked(&self) -> &INTR_HOST_EP_MASKED {
        &self.intr_host_ep_masked
    }
    #[doc = "0xb00 - Host DMA Enable Register"]
    #[inline(always)]
    pub const fn host_dma_enbl(&self) -> &HOST_DMA_ENBL {
        &self.host_dma_enbl
    }
    #[doc = "0xb20 - Host Endpoint 1 Block Register"]
    #[inline(always)]
    pub const fn host_ep1_blk(&self) -> &HOST_EP1_BLK {
        &self.host_ep1_blk
    }
    #[doc = "0xb30 - Host Endpoint 2 Block Register"]
    #[inline(always)]
    pub const fn host_ep2_blk(&self) -> &HOST_EP2_BLK {
        &self.host_ep2_blk
    }
}
#[doc = "HOST_CTL0 (rw) register accessor: Host Control 0 Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctl0`]
module"]
pub type HOST_CTL0 = crate::Reg<host_ctl0::HOST_CTL0_SPEC>;
#[doc = "Host Control 0 Register."]
pub mod host_ctl0;
#[doc = "HOST_CTL1 (rw) register accessor: Host Control 1 Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctl1`]
module"]
pub type HOST_CTL1 = crate::Reg<host_ctl1::HOST_CTL1_SPEC>;
#[doc = "Host Control 1 Register."]
pub mod host_ctl1;
#[doc = "HOST_CTL2 (rw) register accessor: Host Control 2 Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctl2`]
module"]
pub type HOST_CTL2 = crate::Reg<host_ctl2::HOST_CTL2_SPEC>;
#[doc = "Host Control 2 Register."]
pub mod host_ctl2;
#[doc = "HOST_ERR (rw) register accessor: Host Error Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_err`]
module"]
pub type HOST_ERR = crate::Reg<host_err::HOST_ERR_SPEC>;
#[doc = "Host Error Status Register."]
pub mod host_err;
#[doc = "HOST_STATUS (rw) register accessor: Host Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_status`]
module"]
pub type HOST_STATUS = crate::Reg<host_status::HOST_STATUS_SPEC>;
#[doc = "Host Status Register."]
pub mod host_status;
#[doc = "HOST_FCOMP (rw) register accessor: Host SOF Interrupt Frame Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_fcomp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_fcomp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_fcomp`]
module"]
pub type HOST_FCOMP = crate::Reg<host_fcomp::HOST_FCOMP_SPEC>;
#[doc = "Host SOF Interrupt Frame Compare Register"]
pub mod host_fcomp;
#[doc = "HOST_RTIMER (rw) register accessor: Host Retry Timer Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_rtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_rtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_rtimer`]
module"]
pub type HOST_RTIMER = crate::Reg<host_rtimer::HOST_RTIMER_SPEC>;
#[doc = "Host Retry Timer Setup Register"]
pub mod host_rtimer;
#[doc = "HOST_ADDR (rw) register accessor: Host Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_addr`]
module"]
pub type HOST_ADDR = crate::Reg<host_addr::HOST_ADDR_SPEC>;
#[doc = "Host Address Register"]
pub mod host_addr;
#[doc = "HOST_EOF (rw) register accessor: Host EOF Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_eof::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_eof::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_eof`]
module"]
pub type HOST_EOF = crate::Reg<host_eof::HOST_EOF_SPEC>;
#[doc = "Host EOF Setup Register"]
pub mod host_eof;
#[doc = "HOST_FRAME (rw) register accessor: Host Frame Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_frame::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_frame::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_frame`]
module"]
pub type HOST_FRAME = crate::Reg<host_frame::HOST_FRAME_SPEC>;
#[doc = "Host Frame Setup Register"]
pub mod host_frame;
#[doc = "HOST_TOKEN (rw) register accessor: Host Token Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_token::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_token::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_token`]
module"]
pub type HOST_TOKEN = crate::Reg<host_token::HOST_TOKEN_SPEC>;
#[doc = "Host Token Endpoint Register"]
pub mod host_token;
#[doc = "HOST_EP1_CTL (rw) register accessor: Host Endpoint 1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep1_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep1_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep1_ctl`]
module"]
pub type HOST_EP1_CTL = crate::Reg<host_ep1_ctl::HOST_EP1_CTL_SPEC>;
#[doc = "Host Endpoint 1 Control Register"]
pub mod host_ep1_ctl;
#[doc = "HOST_EP1_STATUS (r) register accessor: Host Endpoint 1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep1_status`]
module"]
pub type HOST_EP1_STATUS = crate::Reg<host_ep1_status::HOST_EP1_STATUS_SPEC>;
#[doc = "Host Endpoint 1 Status Register"]
pub mod host_ep1_status;
#[doc = "HOST_EP1_RW1_DR (rw) register accessor: Host Endpoint 1 Data 1-Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep1_rw1_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep1_rw1_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep1_rw1_dr`]
module"]
pub type HOST_EP1_RW1_DR = crate::Reg<host_ep1_rw1_dr::HOST_EP1_RW1_DR_SPEC>;
#[doc = "Host Endpoint 1 Data 1-Byte Register"]
pub mod host_ep1_rw1_dr;
#[doc = "HOST_EP1_RW2_DR (rw) register accessor: Host Endpoint 1 Data 2-Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep1_rw2_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep1_rw2_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep1_rw2_dr`]
module"]
pub type HOST_EP1_RW2_DR = crate::Reg<host_ep1_rw2_dr::HOST_EP1_RW2_DR_SPEC>;
#[doc = "Host Endpoint 1 Data 2-Byte Register"]
pub mod host_ep1_rw2_dr;
#[doc = "HOST_EP2_CTL (rw) register accessor: Host Endpoint 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep2_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep2_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep2_ctl`]
module"]
pub type HOST_EP2_CTL = crate::Reg<host_ep2_ctl::HOST_EP2_CTL_SPEC>;
#[doc = "Host Endpoint 2 Control Register"]
pub mod host_ep2_ctl;
#[doc = "HOST_EP2_STATUS (r) register accessor: Host Endpoint 2 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep2_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep2_status`]
module"]
pub type HOST_EP2_STATUS = crate::Reg<host_ep2_status::HOST_EP2_STATUS_SPEC>;
#[doc = "Host Endpoint 2 Status Register"]
pub mod host_ep2_status;
#[doc = "HOST_EP2_RW1_DR (rw) register accessor: Host Endpoint 2 Data 1-Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep2_rw1_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep2_rw1_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep2_rw1_dr`]
module"]
pub type HOST_EP2_RW1_DR = crate::Reg<host_ep2_rw1_dr::HOST_EP2_RW1_DR_SPEC>;
#[doc = "Host Endpoint 2 Data 1-Byte Register"]
pub mod host_ep2_rw1_dr;
#[doc = "HOST_EP2_RW2_DR (rw) register accessor: Host Endpoint 2 Data 2-Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep2_rw2_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep2_rw2_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep2_rw2_dr`]
module"]
pub type HOST_EP2_RW2_DR = crate::Reg<host_ep2_rw2_dr::HOST_EP2_RW2_DR_SPEC>;
#[doc = "Host Endpoint 2 Data 2-Byte Register"]
pub mod host_ep2_rw2_dr;
#[doc = "HOST_LVL1_SEL (rw) register accessor: Host Interrupt Level 1 Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_lvl1_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_lvl1_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_lvl1_sel`]
module"]
pub type HOST_LVL1_SEL = crate::Reg<host_lvl1_sel::HOST_LVL1_SEL_SPEC>;
#[doc = "Host Interrupt Level 1 Selection Register"]
pub mod host_lvl1_sel;
#[doc = "HOST_LVL2_SEL (rw) register accessor: Host Interrupt Level 2 Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_lvl2_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_lvl2_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_lvl2_sel`]
module"]
pub type HOST_LVL2_SEL = crate::Reg<host_lvl2_sel::HOST_LVL2_SEL_SPEC>;
#[doc = "Host Interrupt Level 2 Selection Register"]
pub mod host_lvl2_sel;
#[doc = "INTR_USBHOST_CAUSE_HI (r) register accessor: Interrupt USB Host Cause High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost_cause_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_cause_hi`]
module"]
pub type INTR_USBHOST_CAUSE_HI = crate::Reg<intr_usbhost_cause_hi::INTR_USBHOST_CAUSE_HI_SPEC>;
#[doc = "Interrupt USB Host Cause High Register"]
pub mod intr_usbhost_cause_hi;
#[doc = "INTR_USBHOST_CAUSE_MED (r) register accessor: Interrupt USB Host Cause Medium Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost_cause_med::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_cause_med`]
module"]
pub type INTR_USBHOST_CAUSE_MED = crate::Reg<intr_usbhost_cause_med::INTR_USBHOST_CAUSE_MED_SPEC>;
#[doc = "Interrupt USB Host Cause Medium Register"]
pub mod intr_usbhost_cause_med;
#[doc = "INTR_USBHOST_CAUSE_LO (r) register accessor: Interrupt USB Host Cause Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost_cause_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_cause_lo`]
module"]
pub type INTR_USBHOST_CAUSE_LO = crate::Reg<intr_usbhost_cause_lo::INTR_USBHOST_CAUSE_LO_SPEC>;
#[doc = "Interrupt USB Host Cause Low Register"]
pub mod intr_usbhost_cause_lo;
#[doc = "INTR_HOST_EP_CAUSE_HI (r) register accessor: Interrupt USB Host Endpoint Cause High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep_cause_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_cause_hi`]
module"]
pub type INTR_HOST_EP_CAUSE_HI = crate::Reg<intr_host_ep_cause_hi::INTR_HOST_EP_CAUSE_HI_SPEC>;
#[doc = "Interrupt USB Host Endpoint Cause High Register"]
pub mod intr_host_ep_cause_hi;
#[doc = "INTR_HOST_EP_CAUSE_MED (r) register accessor: Interrupt USB Host Endpoint Cause Medium Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep_cause_med::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_cause_med`]
module"]
pub type INTR_HOST_EP_CAUSE_MED = crate::Reg<intr_host_ep_cause_med::INTR_HOST_EP_CAUSE_MED_SPEC>;
#[doc = "Interrupt USB Host Endpoint Cause Medium Register"]
pub mod intr_host_ep_cause_med;
#[doc = "INTR_HOST_EP_CAUSE_LO (r) register accessor: Interrupt USB Host Endpoint Cause Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep_cause_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_cause_lo`]
module"]
pub type INTR_HOST_EP_CAUSE_LO = crate::Reg<intr_host_ep_cause_lo::INTR_HOST_EP_CAUSE_LO_SPEC>;
#[doc = "Interrupt USB Host Endpoint Cause Low Register"]
pub mod intr_host_ep_cause_lo;
#[doc = "INTR_USBHOST (rw) register accessor: Interrupt USB Host Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_usbhost::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost`]
module"]
pub type INTR_USBHOST = crate::Reg<intr_usbhost::INTR_USBHOST_SPEC>;
#[doc = "Interrupt USB Host Register"]
pub mod intr_usbhost;
#[doc = "INTR_USBHOST_SET (rw) register accessor: Interrupt USB Host Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_usbhost_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_set`]
module"]
pub type INTR_USBHOST_SET = crate::Reg<intr_usbhost_set::INTR_USBHOST_SET_SPEC>;
#[doc = "Interrupt USB Host Set Register"]
pub mod intr_usbhost_set;
#[doc = "INTR_USBHOST_MASK (rw) register accessor: Interrupt USB Host Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_usbhost_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_mask`]
module"]
pub type INTR_USBHOST_MASK = crate::Reg<intr_usbhost_mask::INTR_USBHOST_MASK_SPEC>;
#[doc = "Interrupt USB Host Mask Register"]
pub mod intr_usbhost_mask;
#[doc = "INTR_USBHOST_MASKED (r) register accessor: Interrupt USB Host Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_masked`]
module"]
pub type INTR_USBHOST_MASKED = crate::Reg<intr_usbhost_masked::INTR_USBHOST_MASKED_SPEC>;
#[doc = "Interrupt USB Host Masked Register"]
pub mod intr_usbhost_masked;
#[doc = "INTR_HOST_EP (rw) register accessor: Interrupt USB Host Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_host_ep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep`]
module"]
pub type INTR_HOST_EP = crate::Reg<intr_host_ep::INTR_HOST_EP_SPEC>;
#[doc = "Interrupt USB Host Endpoint Register"]
pub mod intr_host_ep;
#[doc = "INTR_HOST_EP_SET (rw) register accessor: Interrupt USB Host Endpoint Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_host_ep_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_set`]
module"]
pub type INTR_HOST_EP_SET = crate::Reg<intr_host_ep_set::INTR_HOST_EP_SET_SPEC>;
#[doc = "Interrupt USB Host Endpoint Set Register"]
pub mod intr_host_ep_set;
#[doc = "INTR_HOST_EP_MASK (rw) register accessor: Interrupt USB Host Endpoint Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_host_ep_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_mask`]
module"]
pub type INTR_HOST_EP_MASK = crate::Reg<intr_host_ep_mask::INTR_HOST_EP_MASK_SPEC>;
#[doc = "Interrupt USB Host Endpoint Mask Register"]
pub mod intr_host_ep_mask;
#[doc = "INTR_HOST_EP_MASKED (r) register accessor: Interrupt USB Host Endpoint Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_masked`]
module"]
pub type INTR_HOST_EP_MASKED = crate::Reg<intr_host_ep_masked::INTR_HOST_EP_MASKED_SPEC>;
#[doc = "Interrupt USB Host Endpoint Masked Register"]
pub mod intr_host_ep_masked;
#[doc = "HOST_DMA_ENBL (rw) register accessor: Host DMA Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_dma_enbl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_dma_enbl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_dma_enbl`]
module"]
pub type HOST_DMA_ENBL = crate::Reg<host_dma_enbl::HOST_DMA_ENBL_SPEC>;
#[doc = "Host DMA Enable Register"]
pub mod host_dma_enbl;
#[doc = "HOST_EP1_BLK (rw) register accessor: Host Endpoint 1 Block Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep1_blk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep1_blk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep1_blk`]
module"]
pub type HOST_EP1_BLK = crate::Reg<host_ep1_blk::HOST_EP1_BLK_SPEC>;
#[doc = "Host Endpoint 1 Block Register"]
pub mod host_ep1_blk;
#[doc = "HOST_EP2_BLK (rw) register accessor: Host Endpoint 2 Block Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep2_blk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep2_blk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep2_blk`]
module"]
pub type HOST_EP2_BLK = crate::Reg<host_ep2_blk::HOST_EP2_BLK_SPEC>;
#[doc = "Host Endpoint 2 Block Register"]
pub mod host_ep2_blk;
