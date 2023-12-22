#[doc = "Register `FM_ADDR` reader"]
pub type R = crate::R<FM_ADDR_SPEC>;
#[doc = "Register `FM_ADDR` writer"]
pub type W = crate::W<FM_ADDR_SPEC>;
#[doc = "Field `RA` reader - Row address."]
pub type RA_R = crate::FieldReader<u16>;
#[doc = "Field `RA` writer - Row address."]
pub type RA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BA` reader - Bank address."]
pub type BA_R = crate::FieldReader;
#[doc = "Field `BA` writer - Bank address."]
pub type BA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AXA` reader - Auxiliary address field: 0: regular flash memory. 1: supervisory flash memory."]
pub type AXA_R = crate::BitReader;
#[doc = "Field `AXA` writer - Auxiliary address field: 0: regular flash memory. 1: supervisory flash memory."]
pub type AXA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Row address."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Bank address."]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auxiliary address field: 0: regular flash memory. 1: supervisory flash memory."]
    #[inline(always)]
    pub fn axa(&self) -> AXA_R {
        AXA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Row address."]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<FM_ADDR_SPEC> {
        RA_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Bank address."]
    #[inline(always)]
    #[must_use]
    pub fn ba(&mut self) -> BA_W<FM_ADDR_SPEC> {
        BA_W::new(self, 16)
    }
    #[doc = "Bit 24 - Auxiliary address field: 0: regular flash memory. 1: supervisory flash memory."]
    #[inline(always)]
    #[must_use]
    pub fn axa(&mut self) -> AXA_W<FM_ADDR_SPEC> {
        AXA_W::new(self, 24)
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
#[doc = "Flash macro address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FM_ADDR_SPEC;
impl crate::RegisterSpec for FM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_addr::R`](R) reader structure"]
impl crate::Readable for FM_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fm_addr::W`](W) writer structure"]
impl crate::Writable for FM_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_ADDR to value 0"]
impl crate::Resettable for FM_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
