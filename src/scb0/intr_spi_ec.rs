#[doc = "Register `INTR_SPI_EC` reader"]
pub type R = crate::R<INTR_SPI_EC_SPEC>;
#[doc = "Register `INTR_SPI_EC` writer"]
pub type W = crate::W<INTR_SPI_EC_SPEC>;
#[doc = "Field `WAKE_UP` reader - Wake up request. Active on incoming slave request. Only set when EC_AM is '1'."]
pub type WAKE_UP_R = crate::BitReader;
#[doc = "Field `WAKE_UP` writer - Wake up request. Active on incoming slave request. Only set when EC_AM is '1'."]
pub type WAKE_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_STOP` reader - STOP detection. Activated on the end of a every transfer (SPI deselection). Only set in EZ and CMD_RESP mode and when EC_OP is '1'."]
pub type EZ_STOP_R = crate::BitReader;
#[doc = "Field `EZ_STOP` writer - STOP detection. Activated on the end of a every transfer (SPI deselection). Only set in EZ and CMD_RESP mode and when EC_OP is '1'."]
pub type EZ_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_WRITE_STOP` reader - STOP detection after a write transfer occurred. Activated on the end of a write transfer (SPI deselection). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only set in EZ and CMD_RESP modes and when EC_OP is '1'."]
pub type EZ_WRITE_STOP_R = crate::BitReader;
#[doc = "Field `EZ_WRITE_STOP` writer - STOP detection after a write transfer occurred. Activated on the end of a write transfer (SPI deselection). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only set in EZ and CMD_RESP modes and when EC_OP is '1'."]
pub type EZ_WRITE_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_READ_STOP` reader - STOP detection after a read transfer occurred. Activated on the end of a read transfer (SPI deselection). This event is an indication that a buffer memory location has been read from. Only set in EZ and CMD_RESP modes and when EC_OP is '1'."]
pub type EZ_READ_STOP_R = crate::BitReader;
#[doc = "Field `EZ_READ_STOP` writer - STOP detection after a read transfer occurred. Activated on the end of a read transfer (SPI deselection). This event is an indication that a buffer memory location has been read from. Only set in EZ and CMD_RESP modes and when EC_OP is '1'."]
pub type EZ_READ_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wake up request. Active on incoming slave request. Only set when EC_AM is '1'."]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP detection. Activated on the end of a every transfer (SPI deselection). Only set in EZ and CMD_RESP mode and when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EZ_STOP_R {
        EZ_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP detection after a write transfer occurred. Activated on the end of a write transfer (SPI deselection). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only set in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EZ_WRITE_STOP_R {
        EZ_WRITE_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP detection after a read transfer occurred. Activated on the end of a read transfer (SPI deselection). This event is an indication that a buffer memory location has been read from. Only set in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EZ_READ_STOP_R {
        EZ_READ_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake up request. Active on incoming slave request. Only set when EC_AM is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn wake_up(&mut self) -> WAKE_UP_W<INTR_SPI_EC_SPEC> {
        WAKE_UP_W::new(self, 0)
    }
    #[doc = "Bit 1 - STOP detection. Activated on the end of a every transfer (SPI deselection). Only set in EZ and CMD_RESP mode and when EC_OP is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_stop(&mut self) -> EZ_STOP_W<INTR_SPI_EC_SPEC> {
        EZ_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - STOP detection after a write transfer occurred. Activated on the end of a write transfer (SPI deselection). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only set in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_write_stop(&mut self) -> EZ_WRITE_STOP_W<INTR_SPI_EC_SPEC> {
        EZ_WRITE_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - STOP detection after a read transfer occurred. Activated on the end of a read transfer (SPI deselection). This event is an indication that a buffer memory location has been read from. Only set in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_read_stop(&mut self) -> EZ_READ_STOP_W<INTR_SPI_EC_SPEC> {
        EZ_READ_STOP_W::new(self, 3)
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
#[doc = "Externally clocked SPI interrupt request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_spi_ec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_spi_ec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPI_EC_SPEC;
impl crate::RegisterSpec for INTR_SPI_EC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_spi_ec::R`](R) reader structure"]
impl crate::Readable for INTR_SPI_EC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_spi_ec::W`](W) writer structure"]
impl crate::Writable for INTR_SPI_EC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SPI_EC to value 0"]
impl crate::Resettable for INTR_SPI_EC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
