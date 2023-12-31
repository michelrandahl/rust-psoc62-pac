#[doc = "Register `FM_PL_WRDATA_ALL` reader"]
pub type R = crate::R<FM_PL_WRDATA_ALL_SPEC>;
#[doc = "Register `FM_PL_WRDATA_ALL` writer"]
pub type W = crate::W<FM_PL_WRDATA_ALL_SPEC>;
#[doc = "Field `DATA32` reader - Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
pub type DATA32_R = crate::FieldReader<u32>;
#[doc = "Field `DATA32` writer - Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
pub type DATA32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn data32(&mut self) -> DATA32_W<FM_PL_WRDATA_ALL_SPEC> {
        DATA32_W::new(self, 0)
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
#[doc = "Flash macro write page latches all\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_pl_wrdata_all::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_pl_wrdata_all::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FM_PL_WRDATA_ALL_SPEC;
impl crate::RegisterSpec for FM_PL_WRDATA_ALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_pl_wrdata_all::R`](R) reader structure"]
impl crate::Readable for FM_PL_WRDATA_ALL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fm_pl_wrdata_all::W`](W) writer structure"]
impl crate::Writable for FM_PL_WRDATA_ALL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_PL_WRDATA_ALL to value 0"]
impl crate::Resettable for FM_PL_WRDATA_ALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
