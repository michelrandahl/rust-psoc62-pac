#[doc = "Register `PW_SEQ12` reader"]
pub type R = crate::R<PW_SEQ12_SPEC>;
#[doc = "Register `PW_SEQ12` writer"]
pub type W = crate::W<PW_SEQ12_SPEC>;
#[doc = "Field `PW_SEQ1` reader - Seq1 delay"]
pub type PW_SEQ1_R = crate::FieldReader<u16>;
#[doc = "Field `PW_SEQ1` writer - Seq1 delay"]
pub type PW_SEQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PW_SEQ2_PRE` reader - Seq2 pre delay"]
pub type PW_SEQ2_PRE_R = crate::FieldReader<u16>;
#[doc = "Field `PW_SEQ2_PRE` writer - Seq2 pre delay"]
pub type PW_SEQ2_PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Seq1 delay"]
    #[inline(always)]
    pub fn pw_seq1(&self) -> PW_SEQ1_R {
        PW_SEQ1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Seq2 pre delay"]
    #[inline(always)]
    pub fn pw_seq2_pre(&self) -> PW_SEQ2_PRE_R {
        PW_SEQ2_PRE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Seq1 delay"]
    #[inline(always)]
    #[must_use]
    pub fn pw_seq1(&mut self) -> PW_SEQ1_W<PW_SEQ12_SPEC> {
        PW_SEQ1_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Seq2 pre delay"]
    #[inline(always)]
    #[must_use]
    pub fn pw_seq2_pre(&mut self) -> PW_SEQ2_PRE_W<PW_SEQ12_SPEC> {
        PW_SEQ2_PRE_W::new(self, 16)
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
#[doc = "HV Pulse Delay for seq 1&amp;2 pre\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pw_seq12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pw_seq12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PW_SEQ12_SPEC;
impl crate::RegisterSpec for PW_SEQ12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pw_seq12::R`](R) reader structure"]
impl crate::Readable for PW_SEQ12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pw_seq12::W`](W) writer structure"]
impl crate::Writable for PW_SEQ12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PW_SEQ12 to value 0"]
impl crate::Resettable for PW_SEQ12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
