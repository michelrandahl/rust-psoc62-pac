#[doc = "Register `RX_MATCH` reader"]
pub type R = crate::R<RX_MATCH_SPEC>;
#[doc = "Register `RX_MATCH` writer"]
pub type W = crate::W<RX_MATCH_SPEC>;
#[doc = "Field `ADDR` reader - N/A"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - N/A"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MASK` reader - N/A"]
pub type MASK_R = crate::FieldReader;
#[doc = "Field `MASK` writer - N/A"]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<RX_MATCH_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<RX_MATCH_SPEC> {
        MASK_W::new(self, 16)
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
#[doc = "Slave address and mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_match::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_match::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_MATCH_SPEC;
impl crate::RegisterSpec for RX_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_match::R`](R) reader structure"]
impl crate::Readable for RX_MATCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_match::W`](W) writer structure"]
impl crate::Writable for RX_MATCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_MATCH to value 0"]
impl crate::Resettable for RX_MATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
