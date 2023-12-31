#[doc = "Register `INTR_RX_MASK` reader"]
pub type R = crate::R<INTR_RX_MASK_SPEC>;
#[doc = "Register `INTR_RX_MASK` writer"]
pub type W = crate::W<INTR_RX_MASK_SPEC>;
#[doc = "Field `TRIGGER` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TRIGGER_R = crate::BitReader;
#[doc = "Field `TRIGGER` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_EMPTY` reader - Mask bit for corresponding bit in interrupt request register."]
pub type NOT_EMPTY_R = crate::BitReader;
#[doc = "Field `NOT_EMPTY` writer - Mask bit for corresponding bit in interrupt request register."]
pub type NOT_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL` reader - Mask bit for corresponding bit in interrupt request register."]
pub type FULL_R = crate::BitReader;
#[doc = "Field `FULL` writer - Mask bit for corresponding bit in interrupt request register."]
pub type FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type OVERFLOW_R = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type UNDERFLOW_R = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type UNDERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKED` reader - Mask bit for corresponding bit in interrupt request register."]
pub type BLOCKED_R = crate::BitReader;
#[doc = "Field `BLOCKED` writer - Mask bit for corresponding bit in interrupt request register."]
pub type BLOCKED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_ERROR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type FRAME_ERROR_R = crate::BitReader;
#[doc = "Field `FRAME_ERROR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type FRAME_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERROR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type PARITY_ERROR_R = crate::BitReader;
#[doc = "Field `PARITY_ERROR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type PARITY_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BAUD_DETECT` reader - Mask bit for corresponding bit in interrupt request register."]
pub type BAUD_DETECT_R = crate::BitReader;
#[doc = "Field `BAUD_DETECT` writer - Mask bit for corresponding bit in interrupt request register."]
pub type BAUD_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREAK_DETECT` reader - Mask bit for corresponding bit in interrupt request register."]
pub type BREAK_DETECT_R = crate::BitReader;
#[doc = "Field `BREAK_DETECT` writer - Mask bit for corresponding bit in interrupt request register."]
pub type BREAK_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn not_empty(&self) -> NOT_EMPTY_R {
        NOT_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn frame_error(&self) -> FRAME_ERROR_R {
        FRAME_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn parity_error(&self) -> PARITY_ERROR_R {
        PARITY_ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BAUD_DETECT_R {
        BAUD_DETECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn break_detect(&self) -> BREAK_DETECT_R {
        BREAK_DETECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TRIGGER_W<INTR_RX_MASK_SPEC> {
        TRIGGER_W::new(self, 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn not_empty(&mut self) -> NOT_EMPTY_W<INTR_RX_MASK_SPEC> {
        NOT_EMPTY_W::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FULL_W<INTR_RX_MASK_SPEC> {
        FULL_W::new(self, 3)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<INTR_RX_MASK_SPEC> {
        OVERFLOW_W::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UNDERFLOW_W<INTR_RX_MASK_SPEC> {
        UNDERFLOW_W::new(self, 6)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn blocked(&mut self) -> BLOCKED_W<INTR_RX_MASK_SPEC> {
        BLOCKED_W::new(self, 7)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_error(&mut self) -> FRAME_ERROR_W<INTR_RX_MASK_SPEC> {
        FRAME_ERROR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn parity_error(&mut self) -> PARITY_ERROR_W<INTR_RX_MASK_SPEC> {
        PARITY_ERROR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn baud_detect(&mut self) -> BAUD_DETECT_W<INTR_RX_MASK_SPEC> {
        BAUD_DETECT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn break_detect(&mut self) -> BREAK_DETECT_W<INTR_RX_MASK_SPEC> {
        BREAK_DETECT_W::new(self, 11)
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
#[doc = "Receiver interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rx_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rx_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_RX_MASK_SPEC;
impl crate::RegisterSpec for INTR_RX_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_rx_mask::R`](R) reader structure"]
impl crate::Readable for INTR_RX_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_rx_mask::W`](W) writer structure"]
impl crate::Writable for INTR_RX_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_RX_MASK to value 0"]
impl crate::Resettable for INTR_RX_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
