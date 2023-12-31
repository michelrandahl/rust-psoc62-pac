#[doc = "Register `RGRANT_DELAY_ERS` reader"]
pub type R = crate::R<RGRANT_DELAY_ERS_SPEC>;
#[doc = "Register `RGRANT_DELAY_ERS` writer"]
pub type W = crate::W<RGRANT_DELAY_ERS_SPEC>;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ01` reader - ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_SEQ01_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ01` writer - ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_SEQ01_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ12` reader - ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_SEQ12_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ12` writer - ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_SEQ12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ23` reader - ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_SEQ23_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ23` writer - ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_SEQ23_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq01(&self) -> RGRANT_DELAY_ERS_SEQ01_R {
        RGRANT_DELAY_ERS_SEQ01_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq12(&self) -> RGRANT_DELAY_ERS_SEQ12_R {
        RGRANT_DELAY_ERS_SEQ12_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq23(&self) -> RGRANT_DELAY_ERS_SEQ23_R {
        RGRANT_DELAY_ERS_SEQ23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_seq01(&mut self) -> RGRANT_DELAY_ERS_SEQ01_W<RGRANT_DELAY_ERS_SPEC> {
        RGRANT_DELAY_ERS_SEQ01_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_seq12(&mut self) -> RGRANT_DELAY_ERS_SEQ12_W<RGRANT_DELAY_ERS_SPEC> {
        RGRANT_DELAY_ERS_SEQ12_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_seq23(&mut self) -> RGRANT_DELAY_ERS_SEQ23_W<RGRANT_DELAY_ERS_SPEC> {
        RGRANT_DELAY_ERS_SEQ23_W::new(self, 16)
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
#[doc = "R-grant delay for erase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgrant_delay_ers::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgrant_delay_ers::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGRANT_DELAY_ERS_SPEC;
impl crate::RegisterSpec for RGRANT_DELAY_ERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgrant_delay_ers::R`](R) reader structure"]
impl crate::Readable for RGRANT_DELAY_ERS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rgrant_delay_ers::W`](W) writer structure"]
impl crate::Writable for RGRANT_DELAY_ERS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGRANT_DELAY_ERS to value 0"]
impl crate::Resettable for RGRANT_DELAY_ERS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
