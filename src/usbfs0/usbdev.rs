#[doc = r"Register block"]
#[repr(C)]
pub struct USBDEV {
    ep0_dr: [EP0_DR; 8],
    cr0: CR0,
    cr1: CR1,
    sie_ep_int_en: SIE_EP_INT_EN,
    sie_ep_int_sr: SIE_EP_INT_SR,
    sie_ep1_cnt0: SIE_EP1_CNT0,
    sie_ep1_cnt1: SIE_EP1_CNT1,
    sie_ep1_cr0: SIE_EP1_CR0,
    _reserved8: [u8; 0x04],
    usbio_cr0: USBIO_CR0,
    usbio_cr2: USBIO_CR2,
    usbio_cr1: USBIO_CR1,
    _reserved11: [u8; 0x04],
    dyn_reconfig: DYN_RECONFIG,
    _reserved12: [u8; 0x0c],
    sof0: SOF0,
    sof1: SOF1,
    _reserved14: [u8; 0x08],
    sie_ep2_cnt0: SIE_EP2_CNT0,
    sie_ep2_cnt1: SIE_EP2_CNT1,
    sie_ep2_cr0: SIE_EP2_CR0,
    _reserved17: [u8; 0x04],
    osclk_dr0: OSCLK_DR0,
    osclk_dr1: OSCLK_DR1,
    _reserved19: [u8; 0x18],
    ep0_cr: EP0_CR,
    ep0_cnt: EP0_CNT,
    _reserved21: [u8; 0x08],
    sie_ep3_cnt0: SIE_EP3_CNT0,
    sie_ep3_cnt1: SIE_EP3_CNT1,
    sie_ep3_cr0: SIE_EP3_CR0,
    _reserved24: [u8; 0x34],
    sie_ep4_cnt0: SIE_EP4_CNT0,
    sie_ep4_cnt1: SIE_EP4_CNT1,
    sie_ep4_cr0: SIE_EP4_CR0,
    _reserved27: [u8; 0x34],
    sie_ep5_cnt0: SIE_EP5_CNT0,
    sie_ep5_cnt1: SIE_EP5_CNT1,
    sie_ep5_cr0: SIE_EP5_CR0,
    _reserved30: [u8; 0x34],
    sie_ep6_cnt0: SIE_EP6_CNT0,
    sie_ep6_cnt1: SIE_EP6_CNT1,
    sie_ep6_cr0: SIE_EP6_CR0,
    _reserved33: [u8; 0x34],
    sie_ep7_cnt0: SIE_EP7_CNT0,
    sie_ep7_cnt1: SIE_EP7_CNT1,
    sie_ep7_cr0: SIE_EP7_CR0,
    _reserved36: [u8; 0x34],
    sie_ep8_cnt0: SIE_EP8_CNT0,
    sie_ep8_cnt1: SIE_EP8_CNT1,
    sie_ep8_cr0: SIE_EP8_CR0,
    _reserved39: [u8; 0x04],
    arb_ep1_cfg: ARB_EP1_CFG,
    arb_ep1_int_en: ARB_EP1_INT_EN,
    arb_ep1_sr: ARB_EP1_SR,
    _reserved42: [u8; 0x04],
    arb_rw1_wa: ARB_RW1_WA,
    arb_rw1_wa_msb: ARB_RW1_WA_MSB,
    arb_rw1_ra: ARB_RW1_RA,
    arb_rw1_ra_msb: ARB_RW1_RA_MSB,
    arb_rw1_dr: ARB_RW1_DR,
    _reserved47: [u8; 0x0c],
    buf_size: BUF_SIZE,
    _reserved48: [u8; 0x04],
    ep_active: EP_ACTIVE,
    ep_type: EP_TYPE,
    arb_ep2_cfg: ARB_EP2_CFG,
    arb_ep2_int_en: ARB_EP2_INT_EN,
    arb_ep2_sr: ARB_EP2_SR,
    _reserved53: [u8; 0x04],
    arb_rw2_wa: ARB_RW2_WA,
    arb_rw2_wa_msb: ARB_RW2_WA_MSB,
    arb_rw2_ra: ARB_RW2_RA,
    arb_rw2_ra_msb: ARB_RW2_RA_MSB,
    arb_rw2_dr: ARB_RW2_DR,
    _reserved58: [u8; 0x0c],
    arb_cfg: ARB_CFG,
    usb_clk_en: USB_CLK_EN,
    arb_int_en: ARB_INT_EN,
    arb_int_sr: ARB_INT_SR,
    arb_ep3_cfg: ARB_EP3_CFG,
    arb_ep3_int_en: ARB_EP3_INT_EN,
    arb_ep3_sr: ARB_EP3_SR,
    _reserved65: [u8; 0x04],
    arb_rw3_wa: ARB_RW3_WA,
    arb_rw3_wa_msb: ARB_RW3_WA_MSB,
    arb_rw3_ra: ARB_RW3_RA,
    arb_rw3_ra_msb: ARB_RW3_RA_MSB,
    arb_rw3_dr: ARB_RW3_DR,
    _reserved70: [u8; 0x0c],
    cwa: CWA,
    cwa_msb: CWA_MSB,
    _reserved72: [u8; 0x08],
    arb_ep4_cfg: ARB_EP4_CFG,
    arb_ep4_int_en: ARB_EP4_INT_EN,
    arb_ep4_sr: ARB_EP4_SR,
    _reserved75: [u8; 0x04],
    arb_rw4_wa: ARB_RW4_WA,
    arb_rw4_wa_msb: ARB_RW4_WA_MSB,
    arb_rw4_ra: ARB_RW4_RA,
    arb_rw4_ra_msb: ARB_RW4_RA_MSB,
    arb_rw4_dr: ARB_RW4_DR,
    _reserved80: [u8; 0x0c],
    dma_thres: DMA_THRES,
    dma_thres_msb: DMA_THRES_MSB,
    _reserved82: [u8; 0x08],
    arb_ep5_cfg: ARB_EP5_CFG,
    arb_ep5_int_en: ARB_EP5_INT_EN,
    arb_ep5_sr: ARB_EP5_SR,
    _reserved85: [u8; 0x04],
    arb_rw5_wa: ARB_RW5_WA,
    arb_rw5_wa_msb: ARB_RW5_WA_MSB,
    arb_rw5_ra: ARB_RW5_RA,
    arb_rw5_ra_msb: ARB_RW5_RA_MSB,
    arb_rw5_dr: ARB_RW5_DR,
    _reserved90: [u8; 0x0c],
    bus_rst_cnt: BUS_RST_CNT,
    _reserved91: [u8; 0x0c],
    arb_ep6_cfg: ARB_EP6_CFG,
    arb_ep6_int_en: ARB_EP6_INT_EN,
    arb_ep6_sr: ARB_EP6_SR,
    _reserved94: [u8; 0x04],
    arb_rw6_wa: ARB_RW6_WA,
    arb_rw6_wa_msb: ARB_RW6_WA_MSB,
    arb_rw6_ra: ARB_RW6_RA,
    arb_rw6_ra_msb: ARB_RW6_RA_MSB,
    arb_rw6_dr: ARB_RW6_DR,
    _reserved99: [u8; 0x1c],
    arb_ep7_cfg: ARB_EP7_CFG,
    arb_ep7_int_en: ARB_EP7_INT_EN,
    arb_ep7_sr: ARB_EP7_SR,
    _reserved102: [u8; 0x04],
    arb_rw7_wa: ARB_RW7_WA,
    arb_rw7_wa_msb: ARB_RW7_WA_MSB,
    arb_rw7_ra: ARB_RW7_RA,
    arb_rw7_ra_msb: ARB_RW7_RA_MSB,
    arb_rw7_dr: ARB_RW7_DR,
    _reserved107: [u8; 0x1c],
    arb_ep8_cfg: ARB_EP8_CFG,
    arb_ep8_int_en: ARB_EP8_INT_EN,
    arb_ep8_sr: ARB_EP8_SR,
    _reserved110: [u8; 0x04],
    arb_rw8_wa: ARB_RW8_WA,
    arb_rw8_wa_msb: ARB_RW8_WA_MSB,
    arb_rw8_ra: ARB_RW8_RA,
    arb_rw8_ra_msb: ARB_RW8_RA_MSB,
    arb_rw8_dr: ARB_RW8_DR,
    _reserved115: [u8; 0x1c],
    mem_data: [MEM_DATA; 512],
    _reserved116: [u8; 0x0460],
    sof16: SOF16,
    _reserved117: [u8; 0x1c],
    osclk_dr16: OSCLK_DR16,
    _reserved118: [u8; 0x018c],
    arb_rw1_wa16: ARB_RW1_WA16,
    _reserved119: [u8; 0x04],
    arb_rw1_ra16: ARB_RW1_RA16,
    _reserved120: [u8; 0x04],
    arb_rw1_dr16: ARB_RW1_DR16,
    _reserved121: [u8; 0x2c],
    arb_rw2_wa16: ARB_RW2_WA16,
    _reserved122: [u8; 0x04],
    arb_rw2_ra16: ARB_RW2_RA16,
    _reserved123: [u8; 0x04],
    arb_rw2_dr16: ARB_RW2_DR16,
    _reserved124: [u8; 0x2c],
    arb_rw3_wa16: ARB_RW3_WA16,
    _reserved125: [u8; 0x04],
    arb_rw3_ra16: ARB_RW3_RA16,
    _reserved126: [u8; 0x04],
    arb_rw3_dr16: ARB_RW3_DR16,
    _reserved127: [u8; 0x0c],
    cwa16: CWA16,
    _reserved128: [u8; 0x1c],
    arb_rw4_wa16: ARB_RW4_WA16,
    _reserved129: [u8; 0x04],
    arb_rw4_ra16: ARB_RW4_RA16,
    _reserved130: [u8; 0x04],
    arb_rw4_dr16: ARB_RW4_DR16,
    _reserved131: [u8; 0x0c],
    dma_thres16: DMA_THRES16,
    _reserved132: [u8; 0x1c],
    arb_rw5_wa16: ARB_RW5_WA16,
    _reserved133: [u8; 0x04],
    arb_rw5_ra16: ARB_RW5_RA16,
    _reserved134: [u8; 0x04],
    arb_rw5_dr16: ARB_RW5_DR16,
    _reserved135: [u8; 0x2c],
    arb_rw6_wa16: ARB_RW6_WA16,
    _reserved136: [u8; 0x04],
    arb_rw6_ra16: ARB_RW6_RA16,
    _reserved137: [u8; 0x04],
    arb_rw6_dr16: ARB_RW6_DR16,
    _reserved138: [u8; 0x2c],
    arb_rw7_wa16: ARB_RW7_WA16,
    _reserved139: [u8; 0x04],
    arb_rw7_ra16: ARB_RW7_RA16,
    _reserved140: [u8; 0x04],
    arb_rw7_dr16: ARB_RW7_DR16,
    _reserved141: [u8; 0x2c],
    arb_rw8_wa16: ARB_RW8_WA16,
    _reserved142: [u8; 0x04],
    arb_rw8_ra16: ARB_RW8_RA16,
    _reserved143: [u8; 0x04],
    arb_rw8_dr16: ARB_RW8_DR16,
}
impl USBDEV {
    #[doc = "0x00..0x20 - Control End point EP0 Data Register"]
    #[inline(always)]
    pub const fn ep0_dr(&self, n: usize) -> &EP0_DR {
        &self.ep0_dr[n]
    }
    #[doc = "0x20 - USB control 0 Register"]
    #[inline(always)]
    pub const fn cr0(&self) -> &CR0 {
        &self.cr0
    }
    #[doc = "0x24 - USB control 1 Register"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x28 - USB SIE Data Endpoints Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sie_ep_int_en(&self) -> &SIE_EP_INT_EN {
        &self.sie_ep_int_en
    }
    #[doc = "0x2c - USB SIE Data Endpoint Interrupt Status"]
    #[inline(always)]
    pub const fn sie_ep_int_sr(&self) -> &SIE_EP_INT_SR {
        &self.sie_ep_int_sr
    }
    #[doc = "0x30 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep1_cnt0(&self) -> &SIE_EP1_CNT0 {
        &self.sie_ep1_cnt0
    }
    #[doc = "0x34 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep1_cnt1(&self) -> &SIE_EP1_CNT1 {
        &self.sie_ep1_cnt1
    }
    #[doc = "0x38 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep1_cr0(&self) -> &SIE_EP1_CR0 {
        &self.sie_ep1_cr0
    }
    #[doc = "0x40 - USBIO Control 0 Register"]
    #[inline(always)]
    pub const fn usbio_cr0(&self) -> &USBIO_CR0 {
        &self.usbio_cr0
    }
    #[doc = "0x44 - USBIO control 2 Register"]
    #[inline(always)]
    pub const fn usbio_cr2(&self) -> &USBIO_CR2 {
        &self.usbio_cr2
    }
    #[doc = "0x48 - USBIO control 1 Register"]
    #[inline(always)]
    pub const fn usbio_cr1(&self) -> &USBIO_CR1 {
        &self.usbio_cr1
    }
    #[doc = "0x50 - USB Dynamic reconfiguration register"]
    #[inline(always)]
    pub const fn dyn_reconfig(&self) -> &DYN_RECONFIG {
        &self.dyn_reconfig
    }
    #[doc = "0x60 - Start Of Frame Register"]
    #[inline(always)]
    pub const fn sof0(&self) -> &SOF0 {
        &self.sof0
    }
    #[doc = "0x64 - Start Of Frame Register"]
    #[inline(always)]
    pub const fn sof1(&self) -> &SOF1 {
        &self.sof1
    }
    #[doc = "0x70 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep2_cnt0(&self) -> &SIE_EP2_CNT0 {
        &self.sie_ep2_cnt0
    }
    #[doc = "0x74 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep2_cnt1(&self) -> &SIE_EP2_CNT1 {
        &self.sie_ep2_cnt1
    }
    #[doc = "0x78 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep2_cr0(&self) -> &SIE_EP2_CR0 {
        &self.sie_ep2_cr0
    }
    #[doc = "0x80 - Oscillator lock data register 0"]
    #[inline(always)]
    pub const fn osclk_dr0(&self) -> &OSCLK_DR0 {
        &self.osclk_dr0
    }
    #[doc = "0x84 - Oscillator lock data register 1"]
    #[inline(always)]
    pub const fn osclk_dr1(&self) -> &OSCLK_DR1 {
        &self.osclk_dr1
    }
    #[doc = "0xa0 - Endpoint0 control Register"]
    #[inline(always)]
    pub const fn ep0_cr(&self) -> &EP0_CR {
        &self.ep0_cr
    }
    #[doc = "0xa4 - Endpoint0 count Register"]
    #[inline(always)]
    pub const fn ep0_cnt(&self) -> &EP0_CNT {
        &self.ep0_cnt
    }
    #[doc = "0xb0 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep3_cnt0(&self) -> &SIE_EP3_CNT0 {
        &self.sie_ep3_cnt0
    }
    #[doc = "0xb4 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep3_cnt1(&self) -> &SIE_EP3_CNT1 {
        &self.sie_ep3_cnt1
    }
    #[doc = "0xb8 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep3_cr0(&self) -> &SIE_EP3_CR0 {
        &self.sie_ep3_cr0
    }
    #[doc = "0xf0 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep4_cnt0(&self) -> &SIE_EP4_CNT0 {
        &self.sie_ep4_cnt0
    }
    #[doc = "0xf4 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep4_cnt1(&self) -> &SIE_EP4_CNT1 {
        &self.sie_ep4_cnt1
    }
    #[doc = "0xf8 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep4_cr0(&self) -> &SIE_EP4_CR0 {
        &self.sie_ep4_cr0
    }
    #[doc = "0x130 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep5_cnt0(&self) -> &SIE_EP5_CNT0 {
        &self.sie_ep5_cnt0
    }
    #[doc = "0x134 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep5_cnt1(&self) -> &SIE_EP5_CNT1 {
        &self.sie_ep5_cnt1
    }
    #[doc = "0x138 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep5_cr0(&self) -> &SIE_EP5_CR0 {
        &self.sie_ep5_cr0
    }
    #[doc = "0x170 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep6_cnt0(&self) -> &SIE_EP6_CNT0 {
        &self.sie_ep6_cnt0
    }
    #[doc = "0x174 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep6_cnt1(&self) -> &SIE_EP6_CNT1 {
        &self.sie_ep6_cnt1
    }
    #[doc = "0x178 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep6_cr0(&self) -> &SIE_EP6_CR0 {
        &self.sie_ep6_cr0
    }
    #[doc = "0x1b0 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep7_cnt0(&self) -> &SIE_EP7_CNT0 {
        &self.sie_ep7_cnt0
    }
    #[doc = "0x1b4 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep7_cnt1(&self) -> &SIE_EP7_CNT1 {
        &self.sie_ep7_cnt1
    }
    #[doc = "0x1b8 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep7_cr0(&self) -> &SIE_EP7_CR0 {
        &self.sie_ep7_cr0
    }
    #[doc = "0x1f0 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep8_cnt0(&self) -> &SIE_EP8_CNT0 {
        &self.sie_ep8_cnt0
    }
    #[doc = "0x1f4 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep8_cnt1(&self) -> &SIE_EP8_CNT1 {
        &self.sie_ep8_cnt1
    }
    #[doc = "0x1f8 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep8_cr0(&self) -> &SIE_EP8_CR0 {
        &self.sie_ep8_cr0
    }
    #[doc = "0x200 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep1_cfg(&self) -> &ARB_EP1_CFG {
        &self.arb_ep1_cfg
    }
    #[doc = "0x204 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep1_int_en(&self) -> &ARB_EP1_INT_EN {
        &self.arb_ep1_int_en
    }
    #[doc = "0x208 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep1_sr(&self) -> &ARB_EP1_SR {
        &self.arb_ep1_sr
    }
    #[doc = "0x210 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw1_wa(&self) -> &ARB_RW1_WA {
        &self.arb_rw1_wa
    }
    #[doc = "0x214 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw1_wa_msb(&self) -> &ARB_RW1_WA_MSB {
        &self.arb_rw1_wa_msb
    }
    #[doc = "0x218 - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw1_ra(&self) -> &ARB_RW1_RA {
        &self.arb_rw1_ra
    }
    #[doc = "0x21c - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw1_ra_msb(&self) -> &ARB_RW1_RA_MSB {
        &self.arb_rw1_ra_msb
    }
    #[doc = "0x220 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw1_dr(&self) -> &ARB_RW1_DR {
        &self.arb_rw1_dr
    }
    #[doc = "0x230 - Dedicated Endpoint Buffer Size Register *1"]
    #[inline(always)]
    pub const fn buf_size(&self) -> &BUF_SIZE {
        &self.buf_size
    }
    #[doc = "0x238 - Endpoint Active Indication Register *1"]
    #[inline(always)]
    pub const fn ep_active(&self) -> &EP_ACTIVE {
        &self.ep_active
    }
    #[doc = "0x23c - Endpoint Type (IN/OUT) Indication *1"]
    #[inline(always)]
    pub const fn ep_type(&self) -> &EP_TYPE {
        &self.ep_type
    }
    #[doc = "0x240 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep2_cfg(&self) -> &ARB_EP2_CFG {
        &self.arb_ep2_cfg
    }
    #[doc = "0x244 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep2_int_en(&self) -> &ARB_EP2_INT_EN {
        &self.arb_ep2_int_en
    }
    #[doc = "0x248 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep2_sr(&self) -> &ARB_EP2_SR {
        &self.arb_ep2_sr
    }
    #[doc = "0x250 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw2_wa(&self) -> &ARB_RW2_WA {
        &self.arb_rw2_wa
    }
    #[doc = "0x254 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw2_wa_msb(&self) -> &ARB_RW2_WA_MSB {
        &self.arb_rw2_wa_msb
    }
    #[doc = "0x258 - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw2_ra(&self) -> &ARB_RW2_RA {
        &self.arb_rw2_ra
    }
    #[doc = "0x25c - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw2_ra_msb(&self) -> &ARB_RW2_RA_MSB {
        &self.arb_rw2_ra_msb
    }
    #[doc = "0x260 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw2_dr(&self) -> &ARB_RW2_DR {
        &self.arb_rw2_dr
    }
    #[doc = "0x270 - Arbiter Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_cfg(&self) -> &ARB_CFG {
        &self.arb_cfg
    }
    #[doc = "0x274 - USB Block Clock Enable Register"]
    #[inline(always)]
    pub const fn usb_clk_en(&self) -> &USB_CLK_EN {
        &self.usb_clk_en
    }
    #[doc = "0x278 - Arbiter Interrupt Enable *1"]
    #[inline(always)]
    pub const fn arb_int_en(&self) -> &ARB_INT_EN {
        &self.arb_int_en
    }
    #[doc = "0x27c - Arbiter Interrupt Status *1"]
    #[inline(always)]
    pub const fn arb_int_sr(&self) -> &ARB_INT_SR {
        &self.arb_int_sr
    }
    #[doc = "0x280 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep3_cfg(&self) -> &ARB_EP3_CFG {
        &self.arb_ep3_cfg
    }
    #[doc = "0x284 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep3_int_en(&self) -> &ARB_EP3_INT_EN {
        &self.arb_ep3_int_en
    }
    #[doc = "0x288 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep3_sr(&self) -> &ARB_EP3_SR {
        &self.arb_ep3_sr
    }
    #[doc = "0x290 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw3_wa(&self) -> &ARB_RW3_WA {
        &self.arb_rw3_wa
    }
    #[doc = "0x294 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw3_wa_msb(&self) -> &ARB_RW3_WA_MSB {
        &self.arb_rw3_wa_msb
    }
    #[doc = "0x298 - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw3_ra(&self) -> &ARB_RW3_RA {
        &self.arb_rw3_ra
    }
    #[doc = "0x29c - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw3_ra_msb(&self) -> &ARB_RW3_RA_MSB {
        &self.arb_rw3_ra_msb
    }
    #[doc = "0x2a0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw3_dr(&self) -> &ARB_RW3_DR {
        &self.arb_rw3_dr
    }
    #[doc = "0x2b0 - Common Area Write Address *1"]
    #[inline(always)]
    pub const fn cwa(&self) -> &CWA {
        &self.cwa
    }
    #[doc = "0x2b4 - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn cwa_msb(&self) -> &CWA_MSB {
        &self.cwa_msb
    }
    #[doc = "0x2c0 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep4_cfg(&self) -> &ARB_EP4_CFG {
        &self.arb_ep4_cfg
    }
    #[doc = "0x2c4 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep4_int_en(&self) -> &ARB_EP4_INT_EN {
        &self.arb_ep4_int_en
    }
    #[doc = "0x2c8 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep4_sr(&self) -> &ARB_EP4_SR {
        &self.arb_ep4_sr
    }
    #[doc = "0x2d0 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw4_wa(&self) -> &ARB_RW4_WA {
        &self.arb_rw4_wa
    }
    #[doc = "0x2d4 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw4_wa_msb(&self) -> &ARB_RW4_WA_MSB {
        &self.arb_rw4_wa_msb
    }
    #[doc = "0x2d8 - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw4_ra(&self) -> &ARB_RW4_RA {
        &self.arb_rw4_ra
    }
    #[doc = "0x2dc - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw4_ra_msb(&self) -> &ARB_RW4_RA_MSB {
        &self.arb_rw4_ra_msb
    }
    #[doc = "0x2e0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw4_dr(&self) -> &ARB_RW4_DR {
        &self.arb_rw4_dr
    }
    #[doc = "0x2f0 - DMA Burst / Threshold Configuration"]
    #[inline(always)]
    pub const fn dma_thres(&self) -> &DMA_THRES {
        &self.dma_thres
    }
    #[doc = "0x2f4 - DMA Burst / Threshold Configuration"]
    #[inline(always)]
    pub const fn dma_thres_msb(&self) -> &DMA_THRES_MSB {
        &self.dma_thres_msb
    }
    #[doc = "0x300 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep5_cfg(&self) -> &ARB_EP5_CFG {
        &self.arb_ep5_cfg
    }
    #[doc = "0x304 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep5_int_en(&self) -> &ARB_EP5_INT_EN {
        &self.arb_ep5_int_en
    }
    #[doc = "0x308 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep5_sr(&self) -> &ARB_EP5_SR {
        &self.arb_ep5_sr
    }
    #[doc = "0x310 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw5_wa(&self) -> &ARB_RW5_WA {
        &self.arb_rw5_wa
    }
    #[doc = "0x314 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw5_wa_msb(&self) -> &ARB_RW5_WA_MSB {
        &self.arb_rw5_wa_msb
    }
    #[doc = "0x318 - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw5_ra(&self) -> &ARB_RW5_RA {
        &self.arb_rw5_ra
    }
    #[doc = "0x31c - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw5_ra_msb(&self) -> &ARB_RW5_RA_MSB {
        &self.arb_rw5_ra_msb
    }
    #[doc = "0x320 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw5_dr(&self) -> &ARB_RW5_DR {
        &self.arb_rw5_dr
    }
    #[doc = "0x330 - Bus Reset Count Register"]
    #[inline(always)]
    pub const fn bus_rst_cnt(&self) -> &BUS_RST_CNT {
        &self.bus_rst_cnt
    }
    #[doc = "0x340 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep6_cfg(&self) -> &ARB_EP6_CFG {
        &self.arb_ep6_cfg
    }
    #[doc = "0x344 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep6_int_en(&self) -> &ARB_EP6_INT_EN {
        &self.arb_ep6_int_en
    }
    #[doc = "0x348 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep6_sr(&self) -> &ARB_EP6_SR {
        &self.arb_ep6_sr
    }
    #[doc = "0x350 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw6_wa(&self) -> &ARB_RW6_WA {
        &self.arb_rw6_wa
    }
    #[doc = "0x354 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw6_wa_msb(&self) -> &ARB_RW6_WA_MSB {
        &self.arb_rw6_wa_msb
    }
    #[doc = "0x358 - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw6_ra(&self) -> &ARB_RW6_RA {
        &self.arb_rw6_ra
    }
    #[doc = "0x35c - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw6_ra_msb(&self) -> &ARB_RW6_RA_MSB {
        &self.arb_rw6_ra_msb
    }
    #[doc = "0x360 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw6_dr(&self) -> &ARB_RW6_DR {
        &self.arb_rw6_dr
    }
    #[doc = "0x380 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep7_cfg(&self) -> &ARB_EP7_CFG {
        &self.arb_ep7_cfg
    }
    #[doc = "0x384 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep7_int_en(&self) -> &ARB_EP7_INT_EN {
        &self.arb_ep7_int_en
    }
    #[doc = "0x388 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep7_sr(&self) -> &ARB_EP7_SR {
        &self.arb_ep7_sr
    }
    #[doc = "0x390 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw7_wa(&self) -> &ARB_RW7_WA {
        &self.arb_rw7_wa
    }
    #[doc = "0x394 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw7_wa_msb(&self) -> &ARB_RW7_WA_MSB {
        &self.arb_rw7_wa_msb
    }
    #[doc = "0x398 - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw7_ra(&self) -> &ARB_RW7_RA {
        &self.arb_rw7_ra
    }
    #[doc = "0x39c - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw7_ra_msb(&self) -> &ARB_RW7_RA_MSB {
        &self.arb_rw7_ra_msb
    }
    #[doc = "0x3a0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw7_dr(&self) -> &ARB_RW7_DR {
        &self.arb_rw7_dr
    }
    #[doc = "0x3c0 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep8_cfg(&self) -> &ARB_EP8_CFG {
        &self.arb_ep8_cfg
    }
    #[doc = "0x3c4 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep8_int_en(&self) -> &ARB_EP8_INT_EN {
        &self.arb_ep8_int_en
    }
    #[doc = "0x3c8 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep8_sr(&self) -> &ARB_EP8_SR {
        &self.arb_ep8_sr
    }
    #[doc = "0x3d0 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw8_wa(&self) -> &ARB_RW8_WA {
        &self.arb_rw8_wa
    }
    #[doc = "0x3d4 - Endpoint Write Address value *1"]
    #[inline(always)]
    pub const fn arb_rw8_wa_msb(&self) -> &ARB_RW8_WA_MSB {
        &self.arb_rw8_wa_msb
    }
    #[doc = "0x3d8 - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw8_ra(&self) -> &ARB_RW8_RA {
        &self.arb_rw8_ra
    }
    #[doc = "0x3dc - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn arb_rw8_ra_msb(&self) -> &ARB_RW8_RA_MSB {
        &self.arb_rw8_ra_msb
    }
    #[doc = "0x3e0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw8_dr(&self) -> &ARB_RW8_DR {
        &self.arb_rw8_dr
    }
    #[doc = "0x400..0xc00 - DATA"]
    #[inline(always)]
    pub const fn mem_data(&self, n: usize) -> &MEM_DATA {
        &self.mem_data[n]
    }
    #[doc = "0x1060 - Start Of Frame Register"]
    #[inline(always)]
    pub const fn sof16(&self) -> &SOF16 {
        &self.sof16
    }
    #[doc = "0x1080 - Oscillator lock data register"]
    #[inline(always)]
    pub const fn osclk_dr16(&self) -> &OSCLK_DR16 {
        &self.osclk_dr16
    }
    #[doc = "0x1210 - Endpoint Write Address value"]
    #[inline(always)]
    pub const fn arb_rw1_wa16(&self) -> &ARB_RW1_WA16 {
        &self.arb_rw1_wa16
    }
    #[doc = "0x1218 - Endpoint Read Address value"]
    #[inline(always)]
    pub const fn arb_rw1_ra16(&self) -> &ARB_RW1_RA16 {
        &self.arb_rw1_ra16
    }
    #[doc = "0x1220 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw1_dr16(&self) -> &ARB_RW1_DR16 {
        &self.arb_rw1_dr16
    }
    #[doc = "0x1250 - Endpoint Write Address value"]
    #[inline(always)]
    pub const fn arb_rw2_wa16(&self) -> &ARB_RW2_WA16 {
        &self.arb_rw2_wa16
    }
    #[doc = "0x1258 - Endpoint Read Address value"]
    #[inline(always)]
    pub const fn arb_rw2_ra16(&self) -> &ARB_RW2_RA16 {
        &self.arb_rw2_ra16
    }
    #[doc = "0x1260 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw2_dr16(&self) -> &ARB_RW2_DR16 {
        &self.arb_rw2_dr16
    }
    #[doc = "0x1290 - Endpoint Write Address value"]
    #[inline(always)]
    pub const fn arb_rw3_wa16(&self) -> &ARB_RW3_WA16 {
        &self.arb_rw3_wa16
    }
    #[doc = "0x1298 - Endpoint Read Address value"]
    #[inline(always)]
    pub const fn arb_rw3_ra16(&self) -> &ARB_RW3_RA16 {
        &self.arb_rw3_ra16
    }
    #[doc = "0x12a0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw3_dr16(&self) -> &ARB_RW3_DR16 {
        &self.arb_rw3_dr16
    }
    #[doc = "0x12b0 - Common Area Write Address"]
    #[inline(always)]
    pub const fn cwa16(&self) -> &CWA16 {
        &self.cwa16
    }
    #[doc = "0x12d0 - Endpoint Write Address value"]
    #[inline(always)]
    pub const fn arb_rw4_wa16(&self) -> &ARB_RW4_WA16 {
        &self.arb_rw4_wa16
    }
    #[doc = "0x12d8 - Endpoint Read Address value"]
    #[inline(always)]
    pub const fn arb_rw4_ra16(&self) -> &ARB_RW4_RA16 {
        &self.arb_rw4_ra16
    }
    #[doc = "0x12e0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw4_dr16(&self) -> &ARB_RW4_DR16 {
        &self.arb_rw4_dr16
    }
    #[doc = "0x12f0 - DMA Burst / Threshold Configuration"]
    #[inline(always)]
    pub const fn dma_thres16(&self) -> &DMA_THRES16 {
        &self.dma_thres16
    }
    #[doc = "0x1310 - Endpoint Write Address value"]
    #[inline(always)]
    pub const fn arb_rw5_wa16(&self) -> &ARB_RW5_WA16 {
        &self.arb_rw5_wa16
    }
    #[doc = "0x1318 - Endpoint Read Address value"]
    #[inline(always)]
    pub const fn arb_rw5_ra16(&self) -> &ARB_RW5_RA16 {
        &self.arb_rw5_ra16
    }
    #[doc = "0x1320 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw5_dr16(&self) -> &ARB_RW5_DR16 {
        &self.arb_rw5_dr16
    }
    #[doc = "0x1350 - Endpoint Write Address value"]
    #[inline(always)]
    pub const fn arb_rw6_wa16(&self) -> &ARB_RW6_WA16 {
        &self.arb_rw6_wa16
    }
    #[doc = "0x1358 - Endpoint Read Address value"]
    #[inline(always)]
    pub const fn arb_rw6_ra16(&self) -> &ARB_RW6_RA16 {
        &self.arb_rw6_ra16
    }
    #[doc = "0x1360 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw6_dr16(&self) -> &ARB_RW6_DR16 {
        &self.arb_rw6_dr16
    }
    #[doc = "0x1390 - Endpoint Write Address value"]
    #[inline(always)]
    pub const fn arb_rw7_wa16(&self) -> &ARB_RW7_WA16 {
        &self.arb_rw7_wa16
    }
    #[doc = "0x1398 - Endpoint Read Address value"]
    #[inline(always)]
    pub const fn arb_rw7_ra16(&self) -> &ARB_RW7_RA16 {
        &self.arb_rw7_ra16
    }
    #[doc = "0x13a0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw7_dr16(&self) -> &ARB_RW7_DR16 {
        &self.arb_rw7_dr16
    }
    #[doc = "0x13d0 - Endpoint Write Address value"]
    #[inline(always)]
    pub const fn arb_rw8_wa16(&self) -> &ARB_RW8_WA16 {
        &self.arb_rw8_wa16
    }
    #[doc = "0x13d8 - Endpoint Read Address value"]
    #[inline(always)]
    pub const fn arb_rw8_ra16(&self) -> &ARB_RW8_RA16 {
        &self.arb_rw8_ra16
    }
    #[doc = "0x13e0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw8_dr16(&self) -> &ARB_RW8_DR16 {
        &self.arb_rw8_dr16
    }
}
#[doc = "EP0_DR (rw) register accessor: Control End point EP0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_dr`]
module"]
pub type EP0_DR = crate::Reg<ep0_dr::EP0_DR_SPEC>;
#[doc = "Control End point EP0 Data Register"]
pub mod ep0_dr;
#[doc = "CR0 (rw) register accessor: USB control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "USB control 0 Register"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: USB control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "USB control 1 Register"]
pub mod cr1;
#[doc = "SIE_EP_INT_EN (rw) register accessor: USB SIE Data Endpoints Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep_int_en`]
module"]
pub type SIE_EP_INT_EN = crate::Reg<sie_ep_int_en::SIE_EP_INT_EN_SPEC>;
#[doc = "USB SIE Data Endpoints Interrupt Enable Register"]
pub mod sie_ep_int_en;
#[doc = "SIE_EP_INT_SR (rw) register accessor: USB SIE Data Endpoint Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep_int_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep_int_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep_int_sr`]
module"]
pub type SIE_EP_INT_SR = crate::Reg<sie_ep_int_sr::SIE_EP_INT_SR_SPEC>;
#[doc = "USB SIE Data Endpoint Interrupt Status"]
pub mod sie_ep_int_sr;
#[doc = "SIE_EP1_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep1_cnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep1_cnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep1_cnt0`]
module"]
pub type SIE_EP1_CNT0 = crate::Reg<sie_ep1_cnt0::SIE_EP1_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep1_cnt0;
#[doc = "SIE_EP1_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep1_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep1_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep1_cnt1`]
module"]
pub type SIE_EP1_CNT1 = crate::Reg<sie_ep1_cnt1::SIE_EP1_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep1_cnt1;
#[doc = "SIE_EP1_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep1_cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep1_cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep1_cr0`]
module"]
pub type SIE_EP1_CR0 = crate::Reg<sie_ep1_cr0::SIE_EP1_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep1_cr0;
#[doc = "USBIO_CR0 (rw) register accessor: USBIO Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbio_cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbio_cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbio_cr0`]
module"]
pub type USBIO_CR0 = crate::Reg<usbio_cr0::USBIO_CR0_SPEC>;
#[doc = "USBIO Control 0 Register"]
pub mod usbio_cr0;
#[doc = "USBIO_CR2 (rw) register accessor: USBIO control 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbio_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbio_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbio_cr2`]
module"]
pub type USBIO_CR2 = crate::Reg<usbio_cr2::USBIO_CR2_SPEC>;
#[doc = "USBIO control 2 Register"]
pub mod usbio_cr2;
#[doc = "USBIO_CR1 (rw) register accessor: USBIO control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbio_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbio_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbio_cr1`]
module"]
pub type USBIO_CR1 = crate::Reg<usbio_cr1::USBIO_CR1_SPEC>;
#[doc = "USBIO control 1 Register"]
pub mod usbio_cr1;
#[doc = "DYN_RECONFIG (rw) register accessor: USB Dynamic reconfiguration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dyn_reconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dyn_reconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dyn_reconfig`]
module"]
pub type DYN_RECONFIG = crate::Reg<dyn_reconfig::DYN_RECONFIG_SPEC>;
#[doc = "USB Dynamic reconfiguration register"]
pub mod dyn_reconfig;
#[doc = "SOF0 (r) register accessor: Start Of Frame Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sof0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sof0`]
module"]
pub type SOF0 = crate::Reg<sof0::SOF0_SPEC>;
#[doc = "Start Of Frame Register"]
pub mod sof0;
#[doc = "SOF1 (r) register accessor: Start Of Frame Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sof1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sof1`]
module"]
pub type SOF1 = crate::Reg<sof1::SOF1_SPEC>;
#[doc = "Start Of Frame Register"]
pub mod sof1;
#[doc = "SIE_EP2_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep2_cnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep2_cnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep2_cnt0`]
module"]
pub type SIE_EP2_CNT0 = crate::Reg<sie_ep2_cnt0::SIE_EP2_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep2_cnt0;
#[doc = "SIE_EP2_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep2_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep2_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep2_cnt1`]
module"]
pub type SIE_EP2_CNT1 = crate::Reg<sie_ep2_cnt1::SIE_EP2_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep2_cnt1;
#[doc = "SIE_EP2_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep2_cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep2_cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep2_cr0`]
module"]
pub type SIE_EP2_CR0 = crate::Reg<sie_ep2_cr0::SIE_EP2_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep2_cr0;
#[doc = "OSCLK_DR0 (r) register accessor: Oscillator lock data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osclk_dr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osclk_dr0`]
module"]
pub type OSCLK_DR0 = crate::Reg<osclk_dr0::OSCLK_DR0_SPEC>;
#[doc = "Oscillator lock data register 0"]
pub mod osclk_dr0;
#[doc = "OSCLK_DR1 (r) register accessor: Oscillator lock data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osclk_dr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osclk_dr1`]
module"]
pub type OSCLK_DR1 = crate::Reg<osclk_dr1::OSCLK_DR1_SPEC>;
#[doc = "Oscillator lock data register 1"]
pub mod osclk_dr1;
#[doc = "EP0_CR (rw) register accessor: Endpoint0 control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_cr`]
module"]
pub type EP0_CR = crate::Reg<ep0_cr::EP0_CR_SPEC>;
#[doc = "Endpoint0 control Register"]
pub mod ep0_cr;
#[doc = "EP0_CNT (rw) register accessor: Endpoint0 count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_cnt`]
module"]
pub type EP0_CNT = crate::Reg<ep0_cnt::EP0_CNT_SPEC>;
#[doc = "Endpoint0 count Register"]
pub mod ep0_cnt;
#[doc = "SIE_EP3_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep3_cnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep3_cnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep3_cnt0`]
module"]
pub type SIE_EP3_CNT0 = crate::Reg<sie_ep3_cnt0::SIE_EP3_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep3_cnt0;
#[doc = "SIE_EP3_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep3_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep3_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep3_cnt1`]
module"]
pub type SIE_EP3_CNT1 = crate::Reg<sie_ep3_cnt1::SIE_EP3_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep3_cnt1;
#[doc = "SIE_EP3_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep3_cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep3_cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep3_cr0`]
module"]
pub type SIE_EP3_CR0 = crate::Reg<sie_ep3_cr0::SIE_EP3_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep3_cr0;
#[doc = "SIE_EP4_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep4_cnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep4_cnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep4_cnt0`]
module"]
pub type SIE_EP4_CNT0 = crate::Reg<sie_ep4_cnt0::SIE_EP4_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep4_cnt0;
#[doc = "SIE_EP4_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep4_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep4_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep4_cnt1`]
module"]
pub type SIE_EP4_CNT1 = crate::Reg<sie_ep4_cnt1::SIE_EP4_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep4_cnt1;
#[doc = "SIE_EP4_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep4_cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep4_cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep4_cr0`]
module"]
pub type SIE_EP4_CR0 = crate::Reg<sie_ep4_cr0::SIE_EP4_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep4_cr0;
#[doc = "SIE_EP5_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep5_cnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep5_cnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep5_cnt0`]
module"]
pub type SIE_EP5_CNT0 = crate::Reg<sie_ep5_cnt0::SIE_EP5_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep5_cnt0;
#[doc = "SIE_EP5_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep5_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep5_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep5_cnt1`]
module"]
pub type SIE_EP5_CNT1 = crate::Reg<sie_ep5_cnt1::SIE_EP5_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep5_cnt1;
#[doc = "SIE_EP5_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep5_cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep5_cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep5_cr0`]
module"]
pub type SIE_EP5_CR0 = crate::Reg<sie_ep5_cr0::SIE_EP5_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep5_cr0;
#[doc = "SIE_EP6_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep6_cnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep6_cnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep6_cnt0`]
module"]
pub type SIE_EP6_CNT0 = crate::Reg<sie_ep6_cnt0::SIE_EP6_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep6_cnt0;
#[doc = "SIE_EP6_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep6_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep6_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep6_cnt1`]
module"]
pub type SIE_EP6_CNT1 = crate::Reg<sie_ep6_cnt1::SIE_EP6_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep6_cnt1;
#[doc = "SIE_EP6_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep6_cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep6_cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep6_cr0`]
module"]
pub type SIE_EP6_CR0 = crate::Reg<sie_ep6_cr0::SIE_EP6_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep6_cr0;
#[doc = "SIE_EP7_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep7_cnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep7_cnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep7_cnt0`]
module"]
pub type SIE_EP7_CNT0 = crate::Reg<sie_ep7_cnt0::SIE_EP7_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep7_cnt0;
#[doc = "SIE_EP7_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep7_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep7_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep7_cnt1`]
module"]
pub type SIE_EP7_CNT1 = crate::Reg<sie_ep7_cnt1::SIE_EP7_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep7_cnt1;
#[doc = "SIE_EP7_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep7_cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep7_cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep7_cr0`]
module"]
pub type SIE_EP7_CR0 = crate::Reg<sie_ep7_cr0::SIE_EP7_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep7_cr0;
#[doc = "SIE_EP8_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep8_cnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep8_cnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep8_cnt0`]
module"]
pub type SIE_EP8_CNT0 = crate::Reg<sie_ep8_cnt0::SIE_EP8_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep8_cnt0;
#[doc = "SIE_EP8_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep8_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep8_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep8_cnt1`]
module"]
pub type SIE_EP8_CNT1 = crate::Reg<sie_ep8_cnt1::SIE_EP8_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep8_cnt1;
#[doc = "SIE_EP8_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sie_ep8_cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ep8_cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep8_cr0`]
module"]
pub type SIE_EP8_CR0 = crate::Reg<sie_ep8_cr0::SIE_EP8_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep8_cr0;
#[doc = "ARB_EP1_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep1_cfg`]
module"]
pub type ARB_EP1_CFG = crate::Reg<arb_ep1_cfg::ARB_EP1_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep1_cfg;
#[doc = "ARB_EP1_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep1_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep1_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep1_int_en`]
module"]
pub type ARB_EP1_INT_EN = crate::Reg<arb_ep1_int_en::ARB_EP1_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep1_int_en;
#[doc = "ARB_EP1_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep1_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep1_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep1_sr`]
module"]
pub type ARB_EP1_SR = crate::Reg<arb_ep1_sr::ARB_EP1_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep1_sr;
#[doc = "ARB_RW1_WA (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw1_wa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw1_wa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_wa`]
module"]
pub type ARB_RW1_WA = crate::Reg<arb_rw1_wa::ARB_RW1_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw1_wa;
#[doc = "ARB_RW1_WA_MSB (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw1_wa_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw1_wa_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_wa_msb`]
module"]
pub type ARB_RW1_WA_MSB = crate::Reg<arb_rw1_wa_msb::ARB_RW1_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw1_wa_msb;
#[doc = "ARB_RW1_RA (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw1_ra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw1_ra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_ra`]
module"]
pub type ARB_RW1_RA = crate::Reg<arb_rw1_ra::ARB_RW1_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw1_ra;
#[doc = "ARB_RW1_RA_MSB (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw1_ra_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw1_ra_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_ra_msb`]
module"]
pub type ARB_RW1_RA_MSB = crate::Reg<arb_rw1_ra_msb::ARB_RW1_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw1_ra_msb;
#[doc = "ARB_RW1_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw1_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw1_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_dr`]
module"]
pub type ARB_RW1_DR = crate::Reg<arb_rw1_dr::ARB_RW1_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw1_dr;
#[doc = "BUF_SIZE (rw) register accessor: Dedicated Endpoint Buffer Size Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_size`]
module"]
pub type BUF_SIZE = crate::Reg<buf_size::BUF_SIZE_SPEC>;
#[doc = "Dedicated Endpoint Buffer Size Register *1"]
pub mod buf_size;
#[doc = "EP_ACTIVE (rw) register accessor: Endpoint Active Indication Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_active::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_active::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_active`]
module"]
pub type EP_ACTIVE = crate::Reg<ep_active::EP_ACTIVE_SPEC>;
#[doc = "Endpoint Active Indication Register *1"]
pub mod ep_active;
#[doc = "EP_TYPE (rw) register accessor: Endpoint Type (IN/OUT) Indication *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_type`]
module"]
pub type EP_TYPE = crate::Reg<ep_type::EP_TYPE_SPEC>;
#[doc = "Endpoint Type (IN/OUT) Indication *1"]
pub mod ep_type;
#[doc = "ARB_EP2_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep2_cfg`]
module"]
pub type ARB_EP2_CFG = crate::Reg<arb_ep2_cfg::ARB_EP2_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep2_cfg;
#[doc = "ARB_EP2_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep2_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep2_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep2_int_en`]
module"]
pub type ARB_EP2_INT_EN = crate::Reg<arb_ep2_int_en::ARB_EP2_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep2_int_en;
#[doc = "ARB_EP2_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep2_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep2_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep2_sr`]
module"]
pub type ARB_EP2_SR = crate::Reg<arb_ep2_sr::ARB_EP2_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep2_sr;
#[doc = "ARB_RW2_WA (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw2_wa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw2_wa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_wa`]
module"]
pub type ARB_RW2_WA = crate::Reg<arb_rw2_wa::ARB_RW2_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw2_wa;
#[doc = "ARB_RW2_WA_MSB (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw2_wa_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw2_wa_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_wa_msb`]
module"]
pub type ARB_RW2_WA_MSB = crate::Reg<arb_rw2_wa_msb::ARB_RW2_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw2_wa_msb;
#[doc = "ARB_RW2_RA (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw2_ra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw2_ra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_ra`]
module"]
pub type ARB_RW2_RA = crate::Reg<arb_rw2_ra::ARB_RW2_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw2_ra;
#[doc = "ARB_RW2_RA_MSB (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw2_ra_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw2_ra_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_ra_msb`]
module"]
pub type ARB_RW2_RA_MSB = crate::Reg<arb_rw2_ra_msb::ARB_RW2_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw2_ra_msb;
#[doc = "ARB_RW2_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw2_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw2_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_dr`]
module"]
pub type ARB_RW2_DR = crate::Reg<arb_rw2_dr::ARB_RW2_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw2_dr;
#[doc = "ARB_CFG (rw) register accessor: Arbiter Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_cfg`]
module"]
pub type ARB_CFG = crate::Reg<arb_cfg::ARB_CFG_SPEC>;
#[doc = "Arbiter Configuration Register *1"]
pub mod arb_cfg;
#[doc = "USB_CLK_EN (rw) register accessor: USB Block Clock Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_clk_en`]
module"]
pub type USB_CLK_EN = crate::Reg<usb_clk_en::USB_CLK_EN_SPEC>;
#[doc = "USB Block Clock Enable Register"]
pub mod usb_clk_en;
#[doc = "ARB_INT_EN (rw) register accessor: Arbiter Interrupt Enable *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_int_en`]
module"]
pub type ARB_INT_EN = crate::Reg<arb_int_en::ARB_INT_EN_SPEC>;
#[doc = "Arbiter Interrupt Enable *1"]
pub mod arb_int_en;
#[doc = "ARB_INT_SR (r) register accessor: Arbiter Interrupt Status *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_int_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_int_sr`]
module"]
pub type ARB_INT_SR = crate::Reg<arb_int_sr::ARB_INT_SR_SPEC>;
#[doc = "Arbiter Interrupt Status *1"]
pub mod arb_int_sr;
#[doc = "ARB_EP3_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep3_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep3_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep3_cfg`]
module"]
pub type ARB_EP3_CFG = crate::Reg<arb_ep3_cfg::ARB_EP3_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep3_cfg;
#[doc = "ARB_EP3_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep3_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep3_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep3_int_en`]
module"]
pub type ARB_EP3_INT_EN = crate::Reg<arb_ep3_int_en::ARB_EP3_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep3_int_en;
#[doc = "ARB_EP3_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep3_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep3_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep3_sr`]
module"]
pub type ARB_EP3_SR = crate::Reg<arb_ep3_sr::ARB_EP3_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep3_sr;
#[doc = "ARB_RW3_WA (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw3_wa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw3_wa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_wa`]
module"]
pub type ARB_RW3_WA = crate::Reg<arb_rw3_wa::ARB_RW3_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw3_wa;
#[doc = "ARB_RW3_WA_MSB (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw3_wa_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw3_wa_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_wa_msb`]
module"]
pub type ARB_RW3_WA_MSB = crate::Reg<arb_rw3_wa_msb::ARB_RW3_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw3_wa_msb;
#[doc = "ARB_RW3_RA (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw3_ra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw3_ra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_ra`]
module"]
pub type ARB_RW3_RA = crate::Reg<arb_rw3_ra::ARB_RW3_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw3_ra;
#[doc = "ARB_RW3_RA_MSB (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw3_ra_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw3_ra_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_ra_msb`]
module"]
pub type ARB_RW3_RA_MSB = crate::Reg<arb_rw3_ra_msb::ARB_RW3_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw3_ra_msb;
#[doc = "ARB_RW3_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw3_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw3_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_dr`]
module"]
pub type ARB_RW3_DR = crate::Reg<arb_rw3_dr::ARB_RW3_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw3_dr;
#[doc = "CWA (rw) register accessor: Common Area Write Address *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwa`]
module"]
pub type CWA = crate::Reg<cwa::CWA_SPEC>;
#[doc = "Common Area Write Address *1"]
pub mod cwa;
#[doc = "CWA_MSB (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwa_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwa_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwa_msb`]
module"]
pub type CWA_MSB = crate::Reg<cwa_msb::CWA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod cwa_msb;
#[doc = "ARB_EP4_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep4_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep4_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep4_cfg`]
module"]
pub type ARB_EP4_CFG = crate::Reg<arb_ep4_cfg::ARB_EP4_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep4_cfg;
#[doc = "ARB_EP4_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep4_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep4_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep4_int_en`]
module"]
pub type ARB_EP4_INT_EN = crate::Reg<arb_ep4_int_en::ARB_EP4_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep4_int_en;
#[doc = "ARB_EP4_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep4_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep4_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep4_sr`]
module"]
pub type ARB_EP4_SR = crate::Reg<arb_ep4_sr::ARB_EP4_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep4_sr;
#[doc = "ARB_RW4_WA (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw4_wa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw4_wa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_wa`]
module"]
pub type ARB_RW4_WA = crate::Reg<arb_rw4_wa::ARB_RW4_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw4_wa;
#[doc = "ARB_RW4_WA_MSB (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw4_wa_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw4_wa_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_wa_msb`]
module"]
pub type ARB_RW4_WA_MSB = crate::Reg<arb_rw4_wa_msb::ARB_RW4_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw4_wa_msb;
#[doc = "ARB_RW4_RA (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw4_ra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw4_ra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_ra`]
module"]
pub type ARB_RW4_RA = crate::Reg<arb_rw4_ra::ARB_RW4_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw4_ra;
#[doc = "ARB_RW4_RA_MSB (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw4_ra_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw4_ra_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_ra_msb`]
module"]
pub type ARB_RW4_RA_MSB = crate::Reg<arb_rw4_ra_msb::ARB_RW4_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw4_ra_msb;
#[doc = "ARB_RW4_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw4_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw4_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_dr`]
module"]
pub type ARB_RW4_DR = crate::Reg<arb_rw4_dr::ARB_RW4_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw4_dr;
#[doc = "DMA_THRES (rw) register accessor: DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_thres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_thres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_thres`]
module"]
pub type DMA_THRES = crate::Reg<dma_thres::DMA_THRES_SPEC>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres;
#[doc = "DMA_THRES_MSB (rw) register accessor: DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_thres_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_thres_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_thres_msb`]
module"]
pub type DMA_THRES_MSB = crate::Reg<dma_thres_msb::DMA_THRES_MSB_SPEC>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres_msb;
#[doc = "ARB_EP5_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep5_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep5_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep5_cfg`]
module"]
pub type ARB_EP5_CFG = crate::Reg<arb_ep5_cfg::ARB_EP5_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep5_cfg;
#[doc = "ARB_EP5_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep5_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep5_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep5_int_en`]
module"]
pub type ARB_EP5_INT_EN = crate::Reg<arb_ep5_int_en::ARB_EP5_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep5_int_en;
#[doc = "ARB_EP5_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep5_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep5_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep5_sr`]
module"]
pub type ARB_EP5_SR = crate::Reg<arb_ep5_sr::ARB_EP5_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep5_sr;
#[doc = "ARB_RW5_WA (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw5_wa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw5_wa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_wa`]
module"]
pub type ARB_RW5_WA = crate::Reg<arb_rw5_wa::ARB_RW5_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw5_wa;
#[doc = "ARB_RW5_WA_MSB (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw5_wa_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw5_wa_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_wa_msb`]
module"]
pub type ARB_RW5_WA_MSB = crate::Reg<arb_rw5_wa_msb::ARB_RW5_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw5_wa_msb;
#[doc = "ARB_RW5_RA (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw5_ra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw5_ra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_ra`]
module"]
pub type ARB_RW5_RA = crate::Reg<arb_rw5_ra::ARB_RW5_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw5_ra;
#[doc = "ARB_RW5_RA_MSB (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw5_ra_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw5_ra_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_ra_msb`]
module"]
pub type ARB_RW5_RA_MSB = crate::Reg<arb_rw5_ra_msb::ARB_RW5_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw5_ra_msb;
#[doc = "ARB_RW5_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw5_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw5_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_dr`]
module"]
pub type ARB_RW5_DR = crate::Reg<arb_rw5_dr::ARB_RW5_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw5_dr;
#[doc = "BUS_RST_CNT (rw) register accessor: Bus Reset Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_rst_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_rst_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_rst_cnt`]
module"]
pub type BUS_RST_CNT = crate::Reg<bus_rst_cnt::BUS_RST_CNT_SPEC>;
#[doc = "Bus Reset Count Register"]
pub mod bus_rst_cnt;
#[doc = "ARB_EP6_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep6_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep6_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep6_cfg`]
module"]
pub type ARB_EP6_CFG = crate::Reg<arb_ep6_cfg::ARB_EP6_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep6_cfg;
#[doc = "ARB_EP6_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep6_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep6_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep6_int_en`]
module"]
pub type ARB_EP6_INT_EN = crate::Reg<arb_ep6_int_en::ARB_EP6_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep6_int_en;
#[doc = "ARB_EP6_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep6_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep6_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep6_sr`]
module"]
pub type ARB_EP6_SR = crate::Reg<arb_ep6_sr::ARB_EP6_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep6_sr;
#[doc = "ARB_RW6_WA (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw6_wa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw6_wa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_wa`]
module"]
pub type ARB_RW6_WA = crate::Reg<arb_rw6_wa::ARB_RW6_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw6_wa;
#[doc = "ARB_RW6_WA_MSB (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw6_wa_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw6_wa_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_wa_msb`]
module"]
pub type ARB_RW6_WA_MSB = crate::Reg<arb_rw6_wa_msb::ARB_RW6_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw6_wa_msb;
#[doc = "ARB_RW6_RA (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw6_ra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw6_ra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_ra`]
module"]
pub type ARB_RW6_RA = crate::Reg<arb_rw6_ra::ARB_RW6_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw6_ra;
#[doc = "ARB_RW6_RA_MSB (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw6_ra_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw6_ra_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_ra_msb`]
module"]
pub type ARB_RW6_RA_MSB = crate::Reg<arb_rw6_ra_msb::ARB_RW6_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw6_ra_msb;
#[doc = "ARB_RW6_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw6_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw6_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_dr`]
module"]
pub type ARB_RW6_DR = crate::Reg<arb_rw6_dr::ARB_RW6_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw6_dr;
#[doc = "ARB_EP7_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep7_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep7_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep7_cfg`]
module"]
pub type ARB_EP7_CFG = crate::Reg<arb_ep7_cfg::ARB_EP7_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep7_cfg;
#[doc = "ARB_EP7_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep7_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep7_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep7_int_en`]
module"]
pub type ARB_EP7_INT_EN = crate::Reg<arb_ep7_int_en::ARB_EP7_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep7_int_en;
#[doc = "ARB_EP7_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep7_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep7_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep7_sr`]
module"]
pub type ARB_EP7_SR = crate::Reg<arb_ep7_sr::ARB_EP7_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep7_sr;
#[doc = "ARB_RW7_WA (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw7_wa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw7_wa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_wa`]
module"]
pub type ARB_RW7_WA = crate::Reg<arb_rw7_wa::ARB_RW7_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw7_wa;
#[doc = "ARB_RW7_WA_MSB (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw7_wa_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw7_wa_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_wa_msb`]
module"]
pub type ARB_RW7_WA_MSB = crate::Reg<arb_rw7_wa_msb::ARB_RW7_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw7_wa_msb;
#[doc = "ARB_RW7_RA (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw7_ra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw7_ra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_ra`]
module"]
pub type ARB_RW7_RA = crate::Reg<arb_rw7_ra::ARB_RW7_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw7_ra;
#[doc = "ARB_RW7_RA_MSB (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw7_ra_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw7_ra_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_ra_msb`]
module"]
pub type ARB_RW7_RA_MSB = crate::Reg<arb_rw7_ra_msb::ARB_RW7_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw7_ra_msb;
#[doc = "ARB_RW7_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw7_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw7_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_dr`]
module"]
pub type ARB_RW7_DR = crate::Reg<arb_rw7_dr::ARB_RW7_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw7_dr;
#[doc = "ARB_EP8_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep8_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep8_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep8_cfg`]
module"]
pub type ARB_EP8_CFG = crate::Reg<arb_ep8_cfg::ARB_EP8_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep8_cfg;
#[doc = "ARB_EP8_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep8_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep8_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep8_int_en`]
module"]
pub type ARB_EP8_INT_EN = crate::Reg<arb_ep8_int_en::ARB_EP8_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep8_int_en;
#[doc = "ARB_EP8_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep8_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep8_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep8_sr`]
module"]
pub type ARB_EP8_SR = crate::Reg<arb_ep8_sr::ARB_EP8_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep8_sr;
#[doc = "ARB_RW8_WA (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw8_wa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw8_wa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_wa`]
module"]
pub type ARB_RW8_WA = crate::Reg<arb_rw8_wa::ARB_RW8_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw8_wa;
#[doc = "ARB_RW8_WA_MSB (rw) register accessor: Endpoint Write Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw8_wa_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw8_wa_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_wa_msb`]
module"]
pub type ARB_RW8_WA_MSB = crate::Reg<arb_rw8_wa_msb::ARB_RW8_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw8_wa_msb;
#[doc = "ARB_RW8_RA (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw8_ra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw8_ra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_ra`]
module"]
pub type ARB_RW8_RA = crate::Reg<arb_rw8_ra::ARB_RW8_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw8_ra;
#[doc = "ARB_RW8_RA_MSB (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw8_ra_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw8_ra_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_ra_msb`]
module"]
pub type ARB_RW8_RA_MSB = crate::Reg<arb_rw8_ra_msb::ARB_RW8_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw8_ra_msb;
#[doc = "ARB_RW8_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw8_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw8_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_dr`]
module"]
pub type ARB_RW8_DR = crate::Reg<arb_rw8_dr::ARB_RW8_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw8_dr;
#[doc = "MEM_DATA (rw) register accessor: DATA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_data`]
module"]
pub type MEM_DATA = crate::Reg<mem_data::MEM_DATA_SPEC>;
#[doc = "DATA"]
pub mod mem_data;
#[doc = "SOF16 (r) register accessor: Start Of Frame Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sof16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sof16`]
module"]
pub type SOF16 = crate::Reg<sof16::SOF16_SPEC>;
#[doc = "Start Of Frame Register"]
pub mod sof16;
#[doc = "OSCLK_DR16 (r) register accessor: Oscillator lock data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osclk_dr16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osclk_dr16`]
module"]
pub type OSCLK_DR16 = crate::Reg<osclk_dr16::OSCLK_DR16_SPEC>;
#[doc = "Oscillator lock data register"]
pub mod osclk_dr16;
#[doc = "ARB_RW1_WA16 (rw) register accessor: Endpoint Write Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw1_wa16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw1_wa16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_wa16`]
module"]
pub type ARB_RW1_WA16 = crate::Reg<arb_rw1_wa16::ARB_RW1_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw1_wa16;
#[doc = "ARB_RW1_RA16 (rw) register accessor: Endpoint Read Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw1_ra16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw1_ra16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_ra16`]
module"]
pub type ARB_RW1_RA16 = crate::Reg<arb_rw1_ra16::ARB_RW1_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw1_ra16;
#[doc = "ARB_RW1_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw1_dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw1_dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_dr16`]
module"]
pub type ARB_RW1_DR16 = crate::Reg<arb_rw1_dr16::ARB_RW1_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw1_dr16;
#[doc = "ARB_RW2_WA16 (rw) register accessor: Endpoint Write Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw2_wa16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw2_wa16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_wa16`]
module"]
pub type ARB_RW2_WA16 = crate::Reg<arb_rw2_wa16::ARB_RW2_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw2_wa16;
#[doc = "ARB_RW2_RA16 (rw) register accessor: Endpoint Read Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw2_ra16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw2_ra16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_ra16`]
module"]
pub type ARB_RW2_RA16 = crate::Reg<arb_rw2_ra16::ARB_RW2_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw2_ra16;
#[doc = "ARB_RW2_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw2_dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw2_dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_dr16`]
module"]
pub type ARB_RW2_DR16 = crate::Reg<arb_rw2_dr16::ARB_RW2_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw2_dr16;
#[doc = "ARB_RW3_WA16 (rw) register accessor: Endpoint Write Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw3_wa16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw3_wa16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_wa16`]
module"]
pub type ARB_RW3_WA16 = crate::Reg<arb_rw3_wa16::ARB_RW3_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw3_wa16;
#[doc = "ARB_RW3_RA16 (rw) register accessor: Endpoint Read Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw3_ra16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw3_ra16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_ra16`]
module"]
pub type ARB_RW3_RA16 = crate::Reg<arb_rw3_ra16::ARB_RW3_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw3_ra16;
#[doc = "ARB_RW3_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw3_dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw3_dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_dr16`]
module"]
pub type ARB_RW3_DR16 = crate::Reg<arb_rw3_dr16::ARB_RW3_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw3_dr16;
#[doc = "CWA16 (rw) register accessor: Common Area Write Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwa16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwa16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwa16`]
module"]
pub type CWA16 = crate::Reg<cwa16::CWA16_SPEC>;
#[doc = "Common Area Write Address"]
pub mod cwa16;
#[doc = "ARB_RW4_WA16 (rw) register accessor: Endpoint Write Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw4_wa16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw4_wa16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_wa16`]
module"]
pub type ARB_RW4_WA16 = crate::Reg<arb_rw4_wa16::ARB_RW4_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw4_wa16;
#[doc = "ARB_RW4_RA16 (rw) register accessor: Endpoint Read Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw4_ra16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw4_ra16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_ra16`]
module"]
pub type ARB_RW4_RA16 = crate::Reg<arb_rw4_ra16::ARB_RW4_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw4_ra16;
#[doc = "ARB_RW4_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw4_dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw4_dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_dr16`]
module"]
pub type ARB_RW4_DR16 = crate::Reg<arb_rw4_dr16::ARB_RW4_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw4_dr16;
#[doc = "DMA_THRES16 (rw) register accessor: DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_thres16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_thres16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_thres16`]
module"]
pub type DMA_THRES16 = crate::Reg<dma_thres16::DMA_THRES16_SPEC>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres16;
#[doc = "ARB_RW5_WA16 (rw) register accessor: Endpoint Write Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw5_wa16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw5_wa16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_wa16`]
module"]
pub type ARB_RW5_WA16 = crate::Reg<arb_rw5_wa16::ARB_RW5_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw5_wa16;
#[doc = "ARB_RW5_RA16 (rw) register accessor: Endpoint Read Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw5_ra16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw5_ra16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_ra16`]
module"]
pub type ARB_RW5_RA16 = crate::Reg<arb_rw5_ra16::ARB_RW5_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw5_ra16;
#[doc = "ARB_RW5_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw5_dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw5_dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_dr16`]
module"]
pub type ARB_RW5_DR16 = crate::Reg<arb_rw5_dr16::ARB_RW5_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw5_dr16;
#[doc = "ARB_RW6_WA16 (rw) register accessor: Endpoint Write Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw6_wa16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw6_wa16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_wa16`]
module"]
pub type ARB_RW6_WA16 = crate::Reg<arb_rw6_wa16::ARB_RW6_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw6_wa16;
#[doc = "ARB_RW6_RA16 (rw) register accessor: Endpoint Read Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw6_ra16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw6_ra16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_ra16`]
module"]
pub type ARB_RW6_RA16 = crate::Reg<arb_rw6_ra16::ARB_RW6_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw6_ra16;
#[doc = "ARB_RW6_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw6_dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw6_dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_dr16`]
module"]
pub type ARB_RW6_DR16 = crate::Reg<arb_rw6_dr16::ARB_RW6_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw6_dr16;
#[doc = "ARB_RW7_WA16 (rw) register accessor: Endpoint Write Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw7_wa16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw7_wa16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_wa16`]
module"]
pub type ARB_RW7_WA16 = crate::Reg<arb_rw7_wa16::ARB_RW7_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw7_wa16;
#[doc = "ARB_RW7_RA16 (rw) register accessor: Endpoint Read Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw7_ra16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw7_ra16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_ra16`]
module"]
pub type ARB_RW7_RA16 = crate::Reg<arb_rw7_ra16::ARB_RW7_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw7_ra16;
#[doc = "ARB_RW7_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw7_dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw7_dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_dr16`]
module"]
pub type ARB_RW7_DR16 = crate::Reg<arb_rw7_dr16::ARB_RW7_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw7_dr16;
#[doc = "ARB_RW8_WA16 (rw) register accessor: Endpoint Write Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw8_wa16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw8_wa16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_wa16`]
module"]
pub type ARB_RW8_WA16 = crate::Reg<arb_rw8_wa16::ARB_RW8_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw8_wa16;
#[doc = "ARB_RW8_RA16 (rw) register accessor: Endpoint Read Address value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw8_ra16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw8_ra16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_ra16`]
module"]
pub type ARB_RW8_RA16 = crate::Reg<arb_rw8_ra16::ARB_RW8_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw8_ra16;
#[doc = "ARB_RW8_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_rw8_dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_rw8_dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_dr16`]
module"]
pub type ARB_RW8_DR16 = crate::Reg<arb_rw8_dr16::ARB_RW8_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw8_dr16;
