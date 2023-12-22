#[doc = "Register `INTR_RX_SET` reader"]
pub type R = crate::R<INTR_RX_SET_SPEC>;
#[doc = "Register `INTR_RX_SET` writer"]
pub type W = crate::W<INTR_RX_SET_SPEC>;
#[doc = "Field `TRIGGER` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TRIGGER_R = crate::BitReader;
#[doc = "Field `TRIGGER` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_EMPTY` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type NOT_EMPTY_R = crate::BitReader;
#[doc = "Field `NOT_EMPTY` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type NOT_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type FULL_R = crate::BitReader;
#[doc = "Field `FULL` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type OVERFLOW_R = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type UNDERFLOW_R = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type UNDERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKED` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type BLOCKED_R = crate::BitReader;
#[doc = "Field `BLOCKED` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type BLOCKED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_ERROR` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type FRAME_ERROR_R = crate::BitReader;
#[doc = "Field `FRAME_ERROR` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type FRAME_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERROR` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type PARITY_ERROR_R = crate::BitReader;
#[doc = "Field `PARITY_ERROR` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type PARITY_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BAUD_DETECT` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type BAUD_DETECT_R = crate::BitReader;
#[doc = "Field `BAUD_DETECT` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type BAUD_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREAK_DETECT` reader - Write with '1' to set corresponding bit in interrupt status register."]
pub type BREAK_DETECT_R = crate::BitReader;
#[doc = "Field `BREAK_DETECT` writer - Write with '1' to set corresponding bit in interrupt status register."]
pub type BREAK_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn not_empty(&self) -> NOT_EMPTY_R {
        NOT_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn frame_error(&self) -> FRAME_ERROR_R {
        FRAME_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn parity_error(&self) -> PARITY_ERROR_R {
        PARITY_ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BAUD_DETECT_R {
        BAUD_DETECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn break_detect(&self) -> BREAK_DETECT_R {
        BREAK_DETECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TRIGGER_W<INTR_RX_SET_SPEC> {
        TRIGGER_W::new(self, 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn not_empty(&mut self) -> NOT_EMPTY_W<INTR_RX_SET_SPEC> {
        NOT_EMPTY_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FULL_W<INTR_RX_SET_SPEC> {
        FULL_W::new(self, 3)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<INTR_RX_SET_SPEC> {
        OVERFLOW_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UNDERFLOW_W<INTR_RX_SET_SPEC> {
        UNDERFLOW_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn blocked(&mut self) -> BLOCKED_W<INTR_RX_SET_SPEC> {
        BLOCKED_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_error(&mut self) -> FRAME_ERROR_W<INTR_RX_SET_SPEC> {
        FRAME_ERROR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn parity_error(&mut self) -> PARITY_ERROR_W<INTR_RX_SET_SPEC> {
        PARITY_ERROR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn baud_detect(&mut self) -> BAUD_DETECT_W<INTR_RX_SET_SPEC> {
        BAUD_DETECT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    #[must_use]
    pub fn break_detect(&mut self) -> BREAK_DETECT_W<INTR_RX_SET_SPEC> {
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
#[doc = "Receiver interrupt set request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rx_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rx_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_RX_SET_SPEC;
impl crate::RegisterSpec for INTR_RX_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_rx_set::R`](R) reader structure"]
impl crate::Readable for INTR_RX_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_rx_set::W`](W) writer structure"]
impl crate::Writable for INTR_RX_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_RX_SET to value 0"]
impl crate::Resettable for INTR_RX_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
