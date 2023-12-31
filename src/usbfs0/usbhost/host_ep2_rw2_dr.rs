#[doc = "Register `HOST_EP2_RW2_DR` reader"]
pub type R = crate::R<HOST_EP2_RW2_DR_SPEC>;
#[doc = "Register `HOST_EP2_RW2_DR` writer"]
pub type W = crate::W<HOST_EP2_RW2_DR_SPEC>;
#[doc = "Field `BFDT16` reader - Data Register for EP2. The 2-Byte data is valid."]
pub type BFDT16_R = crate::FieldReader<u16>;
#[doc = "Field `BFDT16` writer - Data Register for EP2. The 2-Byte data is valid."]
pub type BFDT16_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register for EP2. The 2-Byte data is valid."]
    #[inline(always)]
    pub fn bfdt16(&self) -> BFDT16_R {
        BFDT16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register for EP2. The 2-Byte data is valid."]
    #[inline(always)]
    #[must_use]
    pub fn bfdt16(&mut self) -> BFDT16_W<HOST_EP2_RW2_DR_SPEC> {
        BFDT16_W::new(self, 0)
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
#[doc = "Host Endpoint 2 Data 2-Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep2_rw2_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep2_rw2_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_EP2_RW2_DR_SPEC;
impl crate::RegisterSpec for HOST_EP2_RW2_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ep2_rw2_dr::R`](R) reader structure"]
impl crate::Readable for HOST_EP2_RW2_DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_ep2_rw2_dr::W`](W) writer structure"]
impl crate::Writable for HOST_EP2_RW2_DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_EP2_RW2_DR to value 0"]
impl crate::Resettable for HOST_EP2_RW2_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
