#[doc = "Register `BREG[%s]` reader"]
pub type R = crate::R<BREG_SPEC>;
#[doc = "Register `BREG[%s]` writer"]
pub type W = crate::W<BREG_SPEC>;
#[doc = "Field `BREG` reader - Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
pub type BREG_R = crate::FieldReader<u32>;
#[doc = "Field `BREG` writer - Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
pub type BREG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
    #[inline(always)]
    pub fn breg(&self) -> BREG_R {
        BREG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
    #[inline(always)]
    #[must_use]
    pub fn breg(&mut self) -> BREG_W<BREG_SPEC> {
        BREG_W::new(self, 0)
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
#[doc = "Backup register region\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`breg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`breg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BREG_SPEC;
impl crate::RegisterSpec for BREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`breg::R`](R) reader structure"]
impl crate::Readable for BREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`breg::W`](W) writer structure"]
impl crate::Writable for BREG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BREG[%s]
to value 0"]
impl crate::Resettable for BREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
