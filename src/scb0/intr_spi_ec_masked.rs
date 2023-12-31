#[doc = "Register `INTR_SPI_EC_MASKED` reader"]
pub type R = crate::R<INTR_SPI_EC_MASKED_SPEC>;
#[doc = "Field `WAKE_UP` reader - Logical and of corresponding request and mask bits."]
pub type WAKE_UP_R = crate::BitReader;
#[doc = "Field `EZ_STOP` reader - Logical and of corresponding request and mask bits."]
pub type EZ_STOP_R = crate::BitReader;
#[doc = "Field `EZ_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub type EZ_WRITE_STOP_R = crate::BitReader;
#[doc = "Field `EZ_READ_STOP` reader - Logical and of corresponding request and mask bits."]
pub type EZ_READ_STOP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EZ_STOP_R {
        EZ_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EZ_WRITE_STOP_R {
        EZ_WRITE_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EZ_READ_STOP_R {
        EZ_READ_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Externally clocked SPI interrupt masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_spi_ec_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPI_EC_MASKED_SPEC;
impl crate::RegisterSpec for INTR_SPI_EC_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_spi_ec_masked::R`](R) reader structure"]
impl crate::Readable for INTR_SPI_EC_MASKED_SPEC {}
#[doc = "`reset()` method sets INTR_SPI_EC_MASKED to value 0"]
impl crate::Resettable for INTR_SPI_EC_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
