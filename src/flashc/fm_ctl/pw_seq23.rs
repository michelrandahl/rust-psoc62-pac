#[doc = "Register `PW_SEQ23` reader"]
pub type R = crate::R<PW_SEQ23_SPEC>;
#[doc = "Register `PW_SEQ23` writer"]
pub type W = crate::W<PW_SEQ23_SPEC>;
#[doc = "Field `PW_SEQ2_POST` reader - Seq2 post delay"]
pub type PW_SEQ2_POST_R = crate::FieldReader<u16>;
#[doc = "Field `PW_SEQ2_POST` writer - Seq2 post delay"]
pub type PW_SEQ2_POST_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PW_SEQ3` reader - Seq3 delay"]
pub type PW_SEQ3_R = crate::FieldReader<u16>;
#[doc = "Field `PW_SEQ3` writer - Seq3 delay"]
pub type PW_SEQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Seq2 post delay"]
    #[inline(always)]
    pub fn pw_seq2_post(&self) -> PW_SEQ2_POST_R {
        PW_SEQ2_POST_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Seq3 delay"]
    #[inline(always)]
    pub fn pw_seq3(&self) -> PW_SEQ3_R {
        PW_SEQ3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Seq2 post delay"]
    #[inline(always)]
    #[must_use]
    pub fn pw_seq2_post(&mut self) -> PW_SEQ2_POST_W<PW_SEQ23_SPEC> {
        PW_SEQ2_POST_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Seq3 delay"]
    #[inline(always)]
    #[must_use]
    pub fn pw_seq3(&mut self) -> PW_SEQ3_W<PW_SEQ23_SPEC> {
        PW_SEQ3_W::new(self, 16)
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
#[doc = "HV Pulse Delay for seq2 post &amp; seq3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pw_seq23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pw_seq23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PW_SEQ23_SPEC;
impl crate::RegisterSpec for PW_SEQ23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pw_seq23::R`](R) reader structure"]
impl crate::Readable for PW_SEQ23_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pw_seq23::W`](W) writer structure"]
impl crate::Writable for PW_SEQ23_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PW_SEQ23 to value 0"]
impl crate::Resettable for PW_SEQ23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
