#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<INTR_MASKED_SPEC>;
#[doc = "Field `TX_TRIGGER` reader - Logical and of corresponding request and mask bits."]
pub type TX_TRIGGER_R = crate::BitReader;
#[doc = "Field `TX_NOT_FULL` reader - Logical and of corresponding request and mask bits."]
pub type TX_NOT_FULL_R = crate::BitReader;
#[doc = "Field `TX_EMPTY` reader - Logical and of corresponding request and mask bits."]
pub type TX_EMPTY_R = crate::BitReader;
#[doc = "Field `TX_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type TX_OVERFLOW_R = crate::BitReader;
#[doc = "Field `TX_UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type TX_UNDERFLOW_R = crate::BitReader;
#[doc = "Field `TX_WD` reader - Logical and of corresponding request and mask bits."]
pub type TX_WD_R = crate::BitReader;
#[doc = "Field `RX_TRIGGER` reader - Logical and of corresponding request and mask bits."]
pub type RX_TRIGGER_R = crate::BitReader;
#[doc = "Field `RX_NOT_EMPTY` reader - Logical and of corresponding request and mask bits."]
pub type RX_NOT_EMPTY_R = crate::BitReader;
#[doc = "Field `RX_FULL` reader - Logical and of corresponding request and mask bits."]
pub type RX_FULL_R = crate::BitReader;
#[doc = "Field `RX_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type RX_OVERFLOW_R = crate::BitReader;
#[doc = "Field `RX_UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type RX_UNDERFLOW_R = crate::BitReader;
#[doc = "Field `RX_WD` reader - Logical and of corresponding request and mask bits."]
pub type RX_WD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_trigger(&self) -> TX_TRIGGER_R {
        TX_TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_not_full(&self) -> TX_NOT_FULL_R {
        TX_NOT_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_overflow(&self) -> TX_OVERFLOW_R {
        TX_OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_underflow(&self) -> TX_UNDERFLOW_R {
        TX_UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_wd(&self) -> TX_WD_R {
        TX_WD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_trigger(&self) -> RX_TRIGGER_R {
        RX_TRIGGER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_not_empty(&self) -> RX_NOT_EMPTY_R {
        RX_NOT_EMPTY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RX_OVERFLOW_R {
        RX_OVERFLOW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_underflow(&self) -> RX_UNDERFLOW_R {
        RX_UNDERFLOW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_wd(&self) -> RX_WD_R {
        RX_WD_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Interrupt masked register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_masked::R`](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
