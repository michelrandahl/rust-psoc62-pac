#[doc = "Register `DATA2[%s]` reader"]
pub type R = crate::R<DATA2_SPEC>;
#[doc = "Register `DATA2[%s]` writer"]
pub type W = crate::W<DATA2_SPEC>;
#[doc = "Field `DATA` reader - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DATA2_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "LCD Pin Data Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA2_SPEC;
impl crate::RegisterSpec for DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data2::R`](R) reader structure"]
impl crate::Readable for DATA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data2::W`](W) writer structure"]
impl crate::Writable for DATA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA2[%s]
to value 0"]
impl crate::Resettable for DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
