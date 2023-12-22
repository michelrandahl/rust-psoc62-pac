#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `RX_TRIGGER` reader - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
pub type RX_TRIGGER_R = crate::BitReader;
#[doc = "Field `RX_TRIGGER` writer - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
pub type RX_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NOT_EMPTY` reader - RX FIFO is not empty."]
pub type RX_NOT_EMPTY_R = crate::BitReader;
#[doc = "Field `RX_NOT_EMPTY` writer - RX FIFO is not empty."]
pub type RX_NOT_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERFLOW` reader - Attempt to write to a full RX FIFO"]
pub type RX_OVERFLOW_R = crate::BitReader;
#[doc = "Field `RX_OVERFLOW` writer - Attempt to write to a full RX FIFO"]
pub type RX_OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_UNDERFLOW` reader - Attempt to read from an empty RX FIFO"]
pub type RX_UNDERFLOW_R = crate::BitReader;
#[doc = "Field `RX_UNDERFLOW` writer - Attempt to read from an empty RX FIFO"]
pub type RX_UNDERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
    #[inline(always)]
    pub fn rx_trigger(&self) -> RX_TRIGGER_R {
        RX_TRIGGER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - RX FIFO is not empty."]
    #[inline(always)]
    pub fn rx_not_empty(&self) -> RX_NOT_EMPTY_R {
        RX_NOT_EMPTY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Attempt to write to a full RX FIFO"]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RX_OVERFLOW_R {
        RX_OVERFLOW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Attempt to read from an empty RX FIFO"]
    #[inline(always)]
    pub fn rx_underflow(&self) -> RX_UNDERFLOW_R {
        RX_UNDERFLOW_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn rx_trigger(&mut self) -> RX_TRIGGER_W<INTR_SPEC> {
        RX_TRIGGER_W::new(self, 16)
    }
    #[doc = "Bit 18 - RX FIFO is not empty."]
    #[inline(always)]
    #[must_use]
    pub fn rx_not_empty(&mut self) -> RX_NOT_EMPTY_W<INTR_SPEC> {
        RX_NOT_EMPTY_W::new(self, 18)
    }
    #[doc = "Bit 21 - Attempt to write to a full RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow(&mut self) -> RX_OVERFLOW_W<INTR_SPEC> {
        RX_OVERFLOW_W::new(self, 21)
    }
    #[doc = "Bit 22 - Attempt to read from an empty RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rx_underflow(&mut self) -> RX_UNDERFLOW_W<INTR_SPEC> {
        RX_UNDERFLOW_W::new(self, 22)
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
#[doc = "Interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
