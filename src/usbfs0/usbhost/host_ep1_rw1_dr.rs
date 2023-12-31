#[doc = "Register `HOST_EP1_RW1_DR` reader"]
pub type R = crate::R<HOST_EP1_RW1_DR_SPEC>;
#[doc = "Register `HOST_EP1_RW1_DR` writer"]
pub type W = crate::W<HOST_EP1_RW1_DR_SPEC>;
#[doc = "Field `BFDT8` reader - Data Register for EP1. The 1-Byte data is valid."]
pub type BFDT8_R = crate::FieldReader;
#[doc = "Field `BFDT8` writer - Data Register for EP1. The 1-Byte data is valid."]
pub type BFDT8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Register for EP1. The 1-Byte data is valid."]
    #[inline(always)]
    pub fn bfdt8(&self) -> BFDT8_R {
        BFDT8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Register for EP1. The 1-Byte data is valid."]
    #[inline(always)]
    #[must_use]
    pub fn bfdt8(&mut self) -> BFDT8_W<HOST_EP1_RW1_DR_SPEC> {
        BFDT8_W::new(self, 0)
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
#[doc = "Host Endpoint 1 Data 1-Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ep1_rw1_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ep1_rw1_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_EP1_RW1_DR_SPEC;
impl crate::RegisterSpec for HOST_EP1_RW1_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ep1_rw1_dr::R`](R) reader structure"]
impl crate::Readable for HOST_EP1_RW1_DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_ep1_rw1_dr::W`](W) writer structure"]
impl crate::Writable for HOST_EP1_RW1_DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_EP1_RW1_DR to value 0"]
impl crate::Resettable for HOST_EP1_RW1_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
