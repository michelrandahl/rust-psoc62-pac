#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `XIP_MODE` reader - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
pub type XIP_MODE_R = crate::BitReader<XIP_MODE_A>;
#[doc = "Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XIP_MODE_A {
    #[doc = "0: '0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    MMIO_MODE = 0,
    #[doc = "1: 1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    XIP_MODE = 1,
}
impl From<XIP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: XIP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl XIP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XIP_MODE_A {
        match self.bits {
            false => XIP_MODE_A::MMIO_MODE,
            true => XIP_MODE_A::XIP_MODE,
        }
    }
    #[doc = "'0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    #[inline(always)]
    pub fn is_mmio_mode(&self) -> bool {
        *self == XIP_MODE_A::MMIO_MODE
    }
    #[doc = "1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    #[inline(always)]
    pub fn is_xip_mode(&self) -> bool {
        *self == XIP_MODE_A::XIP_MODE
    }
}
#[doc = "Field `XIP_MODE` writer - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
pub type XIP_MODE_W<'a, REG> = crate::BitWriter<'a, REG, XIP_MODE_A>;
impl<'a, REG> XIP_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "'0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    #[inline(always)]
    pub fn mmio_mode(self) -> &'a mut crate::W<REG> {
        self.variant(XIP_MODE_A::MMIO_MODE)
    }
    #[doc = "1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    #[inline(always)]
    pub fn xip_mode(self) -> &'a mut crate::W<REG> {
        self.variant(XIP_MODE_A::XIP_MODE)
    }
}
#[doc = "Field `CLOCK_IF_RX_SEL` reader - Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
pub type CLOCK_IF_RX_SEL_R = crate::FieldReader;
#[doc = "Field `CLOCK_IF_RX_SEL` writer - Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
pub type CLOCK_IF_RX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DESELECT_DELAY` reader - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
pub type DESELECT_DELAY_R = crate::FieldReader;
#[doc = "Field `DESELECT_DELAY` writer - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
pub type DESELECT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLOCK` reader - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
pub type BLOCK_R = crate::BitReader<BLOCK_A>;
#[doc = "Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLOCK_A {
    #[doc = "0: 0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    BUS_ERROR = 0,
    #[doc = "1: 1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    WAIT_STATES = 1,
}
impl From<BLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl BLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLOCK_A {
        match self.bits {
            false => BLOCK_A::BUS_ERROR,
            true => BLOCK_A::WAIT_STATES,
        }
    }
    #[doc = "0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    #[inline(always)]
    pub fn is_bus_error(&self) -> bool {
        *self == BLOCK_A::BUS_ERROR
    }
    #[doc = "1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    #[inline(always)]
    pub fn is_wait_states(&self) -> bool {
        *self == BLOCK_A::WAIT_STATES
    }
}
#[doc = "Field `BLOCK` writer - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
pub type BLOCK_W<'a, REG> = crate::BitWriter<'a, REG, BLOCK_A>;
impl<'a, REG> BLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    #[inline(always)]
    pub fn bus_error(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCK_A::BUS_ERROR)
    }
    #[doc = "1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    #[inline(always)]
    pub fn wait_states(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCK_A::WAIT_STATES)
    }
}
#[doc = "Field `ENABLED` reader - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
#[doc = "IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLED_A {
    #[doc = "0: N/A"]
    DISABLED = 0,
    #[doc = "1: N/A"]
    ENABLED = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::DISABLED,
            true => ENABLED_A::ENABLED,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLED_A::DISABLED
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLED_A::ENABLED
    }
}
#[doc = "Field `ENABLED` writer - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG, ENABLED_A>;
impl<'a, REG> ENABLED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLED_A::DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLED_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
    #[inline(always)]
    pub fn xip_mode(&self) -> XIP_MODE_R {
        XIP_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 12:13 - Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
    #[inline(always)]
    pub fn clock_if_rx_sel(&self) -> CLOCK_IF_RX_SEL_R {
        CLOCK_IF_RX_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    pub fn deselect_delay(&self) -> DESELECT_DELAY_R {
        DESELECT_DELAY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
    #[inline(always)]
    #[must_use]
    pub fn xip_mode(&mut self) -> XIP_MODE_W<CTL_SPEC> {
        XIP_MODE_W::new(self, 0)
    }
    #[doc = "Bits 12:13 - Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
    #[inline(always)]
    #[must_use]
    pub fn clock_if_rx_sel(&mut self) -> CLOCK_IF_RX_SEL_W<CTL_SPEC> {
        CLOCK_IF_RX_SEL_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    #[must_use]
    pub fn deselect_delay(&mut self) -> DESELECT_DELAY_W<CTL_SPEC> {
        DESELECT_DELAY_W::new(self, 16)
    }
    #[doc = "Bit 24 - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<CTL_SPEC> {
        BLOCK_W::new(self, 24)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CTL_SPEC> {
        ENABLED_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x3000"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3000;
}
