#[doc = "Register `BOOKMARK` reader"]
pub type R = crate::R<BOOKMARK_SPEC>;
#[doc = "Register `BOOKMARK` writer"]
pub type W = crate::W<BOOKMARK_SPEC>;
#[doc = "Field `BOOKMARK` reader - Used by FW. Keeps the Current HV cycle sequence"]
pub type BOOKMARK_R = crate::FieldReader<u32>;
#[doc = "Field `BOOKMARK` writer - Used by FW. Keeps the Current HV cycle sequence"]
pub type BOOKMARK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    pub fn bookmark(&self) -> BOOKMARK_R {
        BOOKMARK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    #[must_use]
    pub fn bookmark(&mut self) -> BOOKMARK_W<BOOKMARK_SPEC> {
        BOOKMARK_W::new(self, 0)
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
#[doc = "Bookmark register - keeps the current FW HV seq\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bookmark::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bookmark::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOKMARK_SPEC;
impl crate::RegisterSpec for BOOKMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bookmark::R`](R) reader structure"]
impl crate::Readable for BOOKMARK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bookmark::W`](W) writer structure"]
impl crate::Writable for BOOKMARK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOKMARK to value 0"]
impl crate::Resettable for BOOKMARK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
