#[doc = "Register `SPI_CTRL` reader"]
pub type R = crate::R<SPI_CTRL_SPEC>;
#[doc = "Register `SPI_CTRL` writer"]
pub type W = crate::W<SPI_CTRL_SPEC>;
#[doc = "Field `SSEL_CONTINUOUS` reader - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled indiviual data transfers are NOT seperated by slave select deselection as long as there is data in the TX FIFO. If the TX FIFO becomes empty then the slave select will be deselected. When continuous transfers are not enabled individual data frame transfers are always seperated by slave select deselection: independent of the availability of TX FIFO data frames."]
pub type SSEL_CONTINUOUS_R = crate::BitReader;
#[doc = "Field `SSEL_CONTINUOUS` writer - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled indiviual data transfers are NOT seperated by slave select deselection as long as there is data in the TX FIFO. If the TX FIFO becomes empty then the slave select will be deselected. When continuous transfers are not enabled individual data frame transfers are always seperated by slave select deselection: independent of the availability of TX FIFO data frames."]
pub type SSEL_CONTINUOUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELECT_PRECEDE` reader - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the Slave SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the Slave SELECT line that coincides with the transfer of the first data frame bit."]
pub type SELECT_PRECEDE_R = crate::BitReader;
#[doc = "Field `SELECT_PRECEDE` writer - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the Slave SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the Slave SELECT line that coincides with the transfer of the first data frame bit."]
pub type SELECT_PRECEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - N/A"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - N/A"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - N/A"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - N/A"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LATE_MISO_SAMPLE` reader - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies ( for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
pub type LATE_MISO_SAMPLE_R = crate::BitReader;
#[doc = "Field `LATE_MISO_SAMPLE` writer - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies ( for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
pub type LATE_MISO_SAMPLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_CONTINUOUS` reader - N/A"]
pub type SCLK_CONTINUOUS_R = crate::BitReader;
#[doc = "Field `SCLK_CONTINUOUS` writer - N/A"]
pub type SCLK_CONTINUOUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEL_POLARITY0` reader - N/A"]
pub type SSEL_POLARITY0_R = crate::BitReader;
#[doc = "Field `SSEL_POLARITY0` writer - N/A"]
pub type SSEL_POLARITY0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEL_POLARITY1` reader - N/A"]
pub type SSEL_POLARITY1_R = crate::BitReader;
#[doc = "Field `SSEL_POLARITY1` writer - N/A"]
pub type SSEL_POLARITY1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEL_POLARITY2` reader - N/A"]
pub type SSEL_POLARITY2_R = crate::BitReader;
#[doc = "Field `SSEL_POLARITY2` writer - N/A"]
pub type SSEL_POLARITY2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEL_POLARITY3` reader - N/A"]
pub type SSEL_POLARITY3_R = crate::BitReader;
#[doc = "Field `SSEL_POLARITY3` writer - N/A"]
pub type SSEL_POLARITY3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': No local loopback '1': the SPI master MISO line is connected to the SPI master MOSI line. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
pub type LOOPBACK_R = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': No local loopback '1': the SPI master MISO line is connected to the SPI master MOSI line. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
pub type LOOPBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - N/A"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: SPI Motorola submode."]
    SPI_MOTOROLA = 0,
    #[doc = "1: SPI Texas Instruments submode."]
    SPI_TI = 1,
    #[doc = "2: SPI National Semiconducturs submode."]
    SPI_NS = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::SPI_MOTOROLA),
            1 => Some(MODE_A::SPI_TI),
            2 => Some(MODE_A::SPI_NS),
            _ => None,
        }
    }
    #[doc = "SPI Motorola submode."]
    #[inline(always)]
    pub fn is_spi_motorola(&self) -> bool {
        *self == MODE_A::SPI_MOTOROLA
    }
    #[doc = "SPI Texas Instruments submode."]
    #[inline(always)]
    pub fn is_spi_ti(&self) -> bool {
        *self == MODE_A::SPI_TI
    }
    #[doc = "SPI National Semiconducturs submode."]
    #[inline(always)]
    pub fn is_spi_ns(&self) -> bool {
        *self == MODE_A::SPI_NS
    }
}
#[doc = "Field `MODE` writer - N/A"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI Motorola submode."]
    #[inline(always)]
    pub fn spi_motorola(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::SPI_MOTOROLA)
    }
    #[doc = "SPI Texas Instruments submode."]
    #[inline(always)]
    pub fn spi_ti(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::SPI_TI)
    }
    #[doc = "SPI National Semiconducturs submode."]
    #[inline(always)]
    pub fn spi_ns(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::SPI_NS)
    }
}
#[doc = "Field `SSEL` reader - Selects one of the four incoming/outgoing SPI slave select signals: - 0: Slave 0, SSEL\\[0\\]. - 1: Slave 1, SSEL\\[1\\]. - 2: Slave 2, SSEL\\[2\\]. - 3: Slave 3, SSEL\\[3\\]. The SCB should be disabled when changes are made to this field."]
pub type SSEL_R = crate::FieldReader;
#[doc = "Field `SSEL` writer - Selects one of the four incoming/outgoing SPI slave select signals: - 0: Slave 0, SSEL\\[0\\]. - 1: Slave 1, SSEL\\[1\\]. - 2: Slave 2, SSEL\\[2\\]. - 3: Slave 3, SSEL\\[3\\]. The SCB should be disabled when changes are made to this field."]
pub type SSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MASTER_MODE` reader - N/A"]
pub type MASTER_MODE_R = crate::BitReader;
#[doc = "Field `MASTER_MODE` writer - N/A"]
pub type MASTER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled indiviual data transfers are NOT seperated by slave select deselection as long as there is data in the TX FIFO. If the TX FIFO becomes empty then the slave select will be deselected. When continuous transfers are not enabled individual data frame transfers are always seperated by slave select deselection: independent of the availability of TX FIFO data frames."]
    #[inline(always)]
    pub fn ssel_continuous(&self) -> SSEL_CONTINUOUS_R {
        SSEL_CONTINUOUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the Slave SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the Slave SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    pub fn select_precede(&self) -> SELECT_PRECEDE_R {
        SELECT_PRECEDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies ( for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
    #[inline(always)]
    pub fn late_miso_sample(&self) -> LATE_MISO_SAMPLE_R {
        LATE_MISO_SAMPLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn sclk_continuous(&self) -> SCLK_CONTINUOUS_R {
        SCLK_CONTINUOUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity0(&self) -> SSEL_POLARITY0_R {
        SSEL_POLARITY0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity1(&self) -> SSEL_POLARITY1_R {
        SSEL_POLARITY1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity2(&self) -> SSEL_POLARITY2_R {
        SSEL_POLARITY2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity3(&self) -> SSEL_POLARITY3_R {
        SSEL_POLARITY3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': No local loopback '1': the SPI master MISO line is connected to the SPI master MOSI line. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Selects one of the four incoming/outgoing SPI slave select signals: - 0: Slave 0, SSEL\\[0\\]. - 1: Slave 1, SSEL\\[1\\]. - 2: Slave 2, SSEL\\[2\\]. - 3: Slave 3, SSEL\\[3\\]. The SCB should be disabled when changes are made to this field."]
    #[inline(always)]
    pub fn ssel(&self) -> SSEL_R {
        SSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled indiviual data transfers are NOT seperated by slave select deselection as long as there is data in the TX FIFO. If the TX FIFO becomes empty then the slave select will be deselected. When continuous transfers are not enabled individual data frame transfers are always seperated by slave select deselection: independent of the availability of TX FIFO data frames."]
    #[inline(always)]
    #[must_use]
    pub fn ssel_continuous(&mut self) -> SSEL_CONTINUOUS_W<SPI_CTRL_SPEC> {
        SSEL_CONTINUOUS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the Slave SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the Slave SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    #[must_use]
    pub fn select_precede(&mut self) -> SELECT_PRECEDE_W<SPI_CTRL_SPEC> {
        SELECT_PRECEDE_W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<SPI_CTRL_SPEC> {
        CPHA_W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<SPI_CTRL_SPEC> {
        CPOL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies ( for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
    #[inline(always)]
    #[must_use]
    pub fn late_miso_sample(&mut self) -> LATE_MISO_SAMPLE_W<SPI_CTRL_SPEC> {
        LATE_MISO_SAMPLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_continuous(&mut self) -> SCLK_CONTINUOUS_W<SPI_CTRL_SPEC> {
        SCLK_CONTINUOUS_W::new(self, 5)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ssel_polarity0(&mut self) -> SSEL_POLARITY0_W<SPI_CTRL_SPEC> {
        SSEL_POLARITY0_W::new(self, 8)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ssel_polarity1(&mut self) -> SSEL_POLARITY1_W<SPI_CTRL_SPEC> {
        SSEL_POLARITY1_W::new(self, 9)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ssel_polarity2(&mut self) -> SSEL_POLARITY2_W<SPI_CTRL_SPEC> {
        SSEL_POLARITY2_W::new(self, 10)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ssel_polarity3(&mut self) -> SSEL_POLARITY3_W<SPI_CTRL_SPEC> {
        SSEL_POLARITY3_W::new(self, 11)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': No local loopback '1': the SPI master MISO line is connected to the SPI master MOSI line. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LOOPBACK_W<SPI_CTRL_SPEC> {
        LOOPBACK_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<SPI_CTRL_SPEC> {
        MODE_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Selects one of the four incoming/outgoing SPI slave select signals: - 0: Slave 0, SSEL\\[0\\]. - 1: Slave 1, SSEL\\[1\\]. - 2: Slave 2, SSEL\\[2\\]. - 3: Slave 3, SSEL\\[3\\]. The SCB should be disabled when changes are made to this field."]
    #[inline(always)]
    #[must_use]
    pub fn ssel(&mut self) -> SSEL_W<SPI_CTRL_SPEC> {
        SSEL_W::new(self, 26)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn master_mode(&mut self) -> MASTER_MODE_W<SPI_CTRL_SPEC> {
        MASTER_MODE_W::new(self, 31)
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
#[doc = "SPI control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CTRL_SPEC;
impl crate::RegisterSpec for SPI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CTRL to value 0x0300_0000"]
impl crate::Resettable for SPI_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0000;
}
