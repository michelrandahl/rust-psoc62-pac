#[doc = "Register `INTR_S` reader"]
pub type R = crate::R<INTR_S_SPEC>;
#[doc = "Register `INTR_S` writer"]
pub type W = crate::W<INTR_S_SPEC>;
#[doc = "Field `I2C_ARB_LOST` reader - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine aborts the ongoing transfer. SW may decide to clear the TX and RX FIFOs in case of this error."]
pub type I2C_ARB_LOST_R = crate::BitReader;
#[doc = "Field `I2C_ARB_LOST` writer - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine aborts the ongoing transfer. SW may decide to clear the TX and RX FIFOs in case of this error."]
pub type I2C_ARB_LOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_NACK` reader - N/A"]
pub type I2C_NACK_R = crate::BitReader;
#[doc = "Field `I2C_NACK` writer - N/A"]
pub type I2C_NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ACK` reader - N/A"]
pub type I2C_ACK_R = crate::BitReader;
#[doc = "Field `I2C_ACK` writer - N/A"]
pub type I2C_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_WRITE_STOP` reader - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ base address, will not result in this event being detected)."]
pub type I2C_WRITE_STOP_R = crate::BitReader;
#[doc = "Field `I2C_WRITE_STOP` writer - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ base address, will not result in this event being detected)."]
pub type I2C_WRITE_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_STOP` reader - N/A"]
pub type I2C_STOP_R = crate::BitReader;
#[doc = "Field `I2C_STOP` writer - N/A"]
pub type I2C_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_START` reader - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
pub type I2C_START_R = crate::BitReader;
#[doc = "Field `I2C_START` writer - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
pub type I2C_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ADDR_MATCH` reader - N/A"]
pub type I2C_ADDR_MATCH_R = crate::BitReader;
#[doc = "Field `I2C_ADDR_MATCH` writer - N/A"]
pub type I2C_ADDR_MATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_GENERAL` reader - I2C slave general call address received. If CTRL.ADDR_ACCEPT is set the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
pub type I2C_GENERAL_R = crate::BitReader;
#[doc = "Field `I2C_GENERAL` writer - I2C slave general call address received. If CTRL.ADDR_ACCEPT is set the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
pub type I2C_GENERAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_BUS_ERROR` reader - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type I2C_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `I2C_BUS_ERROR` writer - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type I2C_BUS_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_EZ_WRITE_STOP` reader - N/A"]
pub type SPI_EZ_WRITE_STOP_R = crate::BitReader;
#[doc = "Field `SPI_EZ_WRITE_STOP` writer - N/A"]
pub type SPI_EZ_WRITE_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_EZ_STOP` reader - N/A"]
pub type SPI_EZ_STOP_R = crate::BitReader;
#[doc = "Field `SPI_EZ_STOP` writer - N/A"]
pub type SPI_EZ_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_BUS_ERROR` reader - SPI slave deselected at an unexpected time in the SPI transfer. Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type SPI_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `SPI_BUS_ERROR` writer - SPI slave deselected at an unexpected time in the SPI transfer. Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type SPI_BUS_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine aborts the ongoing transfer. SW may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ base address, will not result in this event being detected)."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2C_WRITE_STOP_R {
        I2C_WRITE_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2C_START_R {
        I2C_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2C_ADDR_MATCH_R {
        I2C_ADDR_MATCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C slave general call address received. If CTRL.ADDR_ACCEPT is set the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2C_GENERAL_R {
        I2C_GENERAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2C_BUS_ERROR_R {
        I2C_BUS_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SPI_EZ_WRITE_STOP_R {
        SPI_EZ_WRITE_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SPI_EZ_STOP_R {
        SPI_EZ_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI slave deselected at an unexpected time in the SPI transfer. Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SPI_BUS_ERROR_R {
        SPI_BUS_ERROR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine aborts the ongoing transfer. SW may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_arb_lost(&mut self) -> I2C_ARB_LOST_W<INTR_S_SPEC> {
        I2C_ARB_LOST_W::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_nack(&mut self) -> I2C_NACK_W<INTR_S_SPEC> {
        I2C_NACK_W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ack(&mut self) -> I2C_ACK_W<INTR_S_SPEC> {
        I2C_ACK_W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ base address, will not result in this event being detected)."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_write_stop(&mut self) -> I2C_WRITE_STOP_W<INTR_S_SPEC> {
        I2C_WRITE_STOP_W::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W<INTR_S_SPEC> {
        I2C_STOP_W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_start(&mut self) -> I2C_START_W<INTR_S_SPEC> {
        I2C_START_W::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_addr_match(&mut self) -> I2C_ADDR_MATCH_W<INTR_S_SPEC> {
        I2C_ADDR_MATCH_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C slave general call address received. If CTRL.ADDR_ACCEPT is set the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_general(&mut self) -> I2C_GENERAL_W<INTR_S_SPEC> {
        I2C_GENERAL_W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_bus_error(&mut self) -> I2C_BUS_ERROR_W<INTR_S_SPEC> {
        I2C_BUS_ERROR_W::new(self, 8)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn spi_ez_write_stop(&mut self) -> SPI_EZ_WRITE_STOP_W<INTR_S_SPEC> {
        SPI_EZ_WRITE_STOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn spi_ez_stop(&mut self) -> SPI_EZ_STOP_W<INTR_S_SPEC> {
        SPI_EZ_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - SPI slave deselected at an unexpected time in the SPI transfer. Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    #[must_use]
    pub fn spi_bus_error(&mut self) -> SPI_BUS_ERROR_W<INTR_S_SPEC> {
        SPI_BUS_ERROR_W::new(self, 11)
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
#[doc = "Slave interrupt request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_S_SPEC;
impl crate::RegisterSpec for INTR_S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_s::R`](R) reader structure"]
impl crate::Readable for INTR_S_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_s::W`](W) writer structure"]
impl crate::Writable for INTR_S_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_S to value 0"]
impl crate::Resettable for INTR_S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
