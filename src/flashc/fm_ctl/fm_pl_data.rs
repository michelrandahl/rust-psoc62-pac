#[doc = "Register `FM_PL_DATA[%s]` reader"]
pub type R = crate::R<FM_PL_DATA_SPEC>;
#[doc = "Register `FM_PL_DATA[%s]` writer"]
pub type W = crate::W<FM_PL_DATA_SPEC>;
#[doc = "Field `DATA32` reader - Four page latch Bytes When reading the page latches it requires FM_CTL.IF_SEL to be '1' Note: the high Voltage page latches are readable for test mode functionality."]
pub type DATA32_R = crate::FieldReader<u32>;
#[doc = "Field `DATA32` writer - Four page latch Bytes When reading the page latches it requires FM_CTL.IF_SEL to be '1' Note: the high Voltage page latches are readable for test mode functionality."]
pub type DATA32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Four page latch Bytes When reading the page latches it requires FM_CTL.IF_SEL to be '1' Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four page latch Bytes When reading the page latches it requires FM_CTL.IF_SEL to be '1' Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    #[must_use]
    pub fn data32(&mut self) -> DATA32_W<FM_PL_DATA_SPEC> {
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
#[doc = "Flash macro Page Latches data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_pl_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_pl_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FM_PL_DATA_SPEC;
impl crate::RegisterSpec for FM_PL_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_pl_data::R`](R) reader structure"]
impl crate::Readable for FM_PL_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fm_pl_data::W`](W) writer structure"]
impl crate::Writable for FM_PL_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_PL_DATA[%s]
to value 0"]
impl crate::Resettable for FM_PL_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
