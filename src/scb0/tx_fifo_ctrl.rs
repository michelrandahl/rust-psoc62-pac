#[doc = "Register `TX_FIFO_CTRL` reader"]
pub type R = crate::R<TX_FIFO_CTRL_SPEC>;
#[doc = "Register `TX_FIFO_CTRL` writer"]
pub type W = crate::W<TX_FIFO_CTRL_SPEC>;
#[doc = "Field `TRIGGER_LEVEL` reader - N/A"]
pub type TRIGGER_LEVEL_R = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - N/A"]
pub type TRIGGER_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLEAR` reader - N/A"]
pub type CLEAR_R = crate::BitReader;
#[doc = "Field `CLEAR` writer - N/A"]
pub type CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREEZE` reader - N/A"]
pub type FREEZE_R = crate::BitReader;
#[doc = "Field `FREEZE` writer - N/A"]
pub type FREEZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<TX_FIFO_CTRL_SPEC> {
        TRIGGER_LEVEL_W::new(self, 0)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<TX_FIFO_CTRL_SPEC> {
        CLEAR_W::new(self, 16)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FREEZE_W<TX_FIFO_CTRL_SPEC> {
        FREEZE_W::new(self, 17)
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
#[doc = "Transmitter FIFO control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_fifo_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_fifo_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_FIFO_CTRL_SPEC;
impl crate::RegisterSpec for TX_FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for TX_FIFO_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for TX_FIFO_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_FIFO_CTRL to value 0"]
impl crate::Resettable for TX_FIFO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
