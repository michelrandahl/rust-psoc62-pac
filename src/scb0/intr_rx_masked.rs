#[doc = "Register `INTR_RX_MASKED` reader"]
pub type R = crate::R<INTR_RX_MASKED_SPEC>;
#[doc = "Field `TRIGGER` reader - Logical and of corresponding request and mask bits."]
pub type TRIGGER_R = crate::BitReader;
#[doc = "Field `NOT_EMPTY` reader - Logical and of corresponding request and mask bits."]
pub type NOT_EMPTY_R = crate::BitReader;
#[doc = "Field `FULL` reader - Logical and of corresponding request and mask bits."]
pub type FULL_R = crate::BitReader;
#[doc = "Field `OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type OVERFLOW_R = crate::BitReader;
#[doc = "Field `UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type UNDERFLOW_R = crate::BitReader;
#[doc = "Field `BLOCKED` reader - Logical and of corresponding request and mask bits."]
pub type BLOCKED_R = crate::BitReader;
#[doc = "Field `FRAME_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type FRAME_ERROR_R = crate::BitReader;
#[doc = "Field `PARITY_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type PARITY_ERROR_R = crate::BitReader;
#[doc = "Field `BAUD_DETECT` reader - Logical and of corresponding request and mask bits."]
pub type BAUD_DETECT_R = crate::BitReader;
#[doc = "Field `BREAK_DETECT` reader - Logical and of corresponding request and mask bits."]
pub type BREAK_DETECT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn not_empty(&self) -> NOT_EMPTY_R {
        NOT_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn frame_error(&self) -> FRAME_ERROR_R {
        FRAME_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn parity_error(&self) -> PARITY_ERROR_R {
        PARITY_ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BAUD_DETECT_R {
        BAUD_DETECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn break_detect(&self) -> BREAK_DETECT_R {
        BREAK_DETECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Receiver interrupt masked request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rx_masked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_RX_MASKED_SPEC;
impl crate::RegisterSpec for INTR_RX_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_rx_masked::R`](R) reader structure"]
impl crate::Readable for INTR_RX_MASKED_SPEC {}
#[doc = "`reset()` method sets INTR_RX_MASKED to value 0"]
impl crate::Resettable for INTR_RX_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
