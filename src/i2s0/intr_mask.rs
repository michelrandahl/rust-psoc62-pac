#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<INTR_MASK_SPEC>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<INTR_MASK_SPEC>;
#[doc = "Field `TX_TRIGGER` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TX_TRIGGER_R = crate::BitReader;
#[doc = "Field `TX_TRIGGER` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TX_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NOT_FULL` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TX_NOT_FULL_R = crate::BitReader;
#[doc = "Field `TX_NOT_FULL` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TX_NOT_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TX_EMPTY_R = crate::BitReader;
#[doc = "Field `TX_EMPTY` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TX_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_OVERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TX_OVERFLOW_R = crate::BitReader;
#[doc = "Field `TX_OVERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TX_OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_UNDERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TX_UNDERFLOW_R = crate::BitReader;
#[doc = "Field `TX_UNDERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TX_UNDERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_WD` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TX_WD_R = crate::BitReader;
#[doc = "Field `TX_WD` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TX_WD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TRIGGER` reader - Mask bit for corresponding bit in interrupt request register."]
pub type RX_TRIGGER_R = crate::BitReader;
#[doc = "Field `RX_TRIGGER` writer - Mask bit for corresponding bit in interrupt request register."]
pub type RX_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NOT_EMPTY` reader - Mask bit for corresponding bit in interrupt request register."]
pub type RX_NOT_EMPTY_R = crate::BitReader;
#[doc = "Field `RX_NOT_EMPTY` writer - Mask bit for corresponding bit in interrupt request register."]
pub type RX_NOT_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FULL` reader - Mask bit for corresponding bit in interrupt request register."]
pub type RX_FULL_R = crate::BitReader;
#[doc = "Field `RX_FULL` writer - Mask bit for corresponding bit in interrupt request register."]
pub type RX_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type RX_OVERFLOW_R = crate::BitReader;
#[doc = "Field `RX_OVERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type RX_OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_UNDERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type RX_UNDERFLOW_R = crate::BitReader;
#[doc = "Field `RX_UNDERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type RX_UNDERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_WD` reader - Mask bit for corresponding bit in interrupt request register."]
pub type RX_WD_R = crate::BitReader;
#[doc = "Field `RX_WD` writer - Mask bit for corresponding bit in interrupt request register."]
pub type RX_WD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_trigger(&self) -> TX_TRIGGER_R {
        TX_TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_not_full(&self) -> TX_NOT_FULL_R {
        TX_NOT_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_overflow(&self) -> TX_OVERFLOW_R {
        TX_OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_underflow(&self) -> TX_UNDERFLOW_R {
        TX_UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_wd(&self) -> TX_WD_R {
        TX_WD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_trigger(&self) -> RX_TRIGGER_R {
        RX_TRIGGER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_not_empty(&self) -> RX_NOT_EMPTY_R {
        RX_NOT_EMPTY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RX_OVERFLOW_R {
        RX_OVERFLOW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_underflow(&self) -> RX_UNDERFLOW_R {
        RX_UNDERFLOW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_wd(&self) -> RX_WD_R {
        RX_WD_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_trigger(&mut self) -> TX_TRIGGER_W<INTR_MASK_SPEC> {
        TX_TRIGGER_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_not_full(&mut self) -> TX_NOT_FULL_W<INTR_MASK_SPEC> {
        TX_NOT_FULL_W::new(self, 1)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<INTR_MASK_SPEC> {
        TX_EMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_overflow(&mut self) -> TX_OVERFLOW_W<INTR_MASK_SPEC> {
        TX_OVERFLOW_W::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_underflow(&mut self) -> TX_UNDERFLOW_W<INTR_MASK_SPEC> {
        TX_UNDERFLOW_W::new(self, 6)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wd(&mut self) -> TX_WD_W<INTR_MASK_SPEC> {
        TX_WD_W::new(self, 8)
    }
    #[doc = "Bit 16 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_trigger(&mut self) -> RX_TRIGGER_W<INTR_MASK_SPEC> {
        RX_TRIGGER_W::new(self, 16)
    }
    #[doc = "Bit 18 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_not_empty(&mut self) -> RX_NOT_EMPTY_W<INTR_MASK_SPEC> {
        RX_NOT_EMPTY_W::new(self, 18)
    }
    #[doc = "Bit 19 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<INTR_MASK_SPEC> {
        RX_FULL_W::new(self, 19)
    }
    #[doc = "Bit 21 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow(&mut self) -> RX_OVERFLOW_W<INTR_MASK_SPEC> {
        RX_OVERFLOW_W::new(self, 21)
    }
    #[doc = "Bit 22 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_underflow(&mut self) -> RX_UNDERFLOW_W<INTR_MASK_SPEC> {
        RX_UNDERFLOW_W::new(self, 22)
    }
    #[doc = "Bit 24 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_wd(&mut self) -> RX_WD_W<INTR_MASK_SPEC> {
        RX_WD_W::new(self, 24)
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
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
