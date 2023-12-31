#[doc = "Register `RGRANT_SCALE_ERS` reader"]
pub type R = crate::R<RGRANT_SCALE_ERS_SPEC>;
#[doc = "Register `RGRANT_SCALE_ERS` writer"]
pub type W = crate::W<RGRANT_SCALE_ERS_SPEC>;
#[doc = "Field `SCALE_ERS_SEQ01` reader - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ01_R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_SEQ01` writer - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ01_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_ERS_SEQ12` reader - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ12_R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_SEQ12` writer - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_ERS_SEQ23` reader - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ23_R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_SEQ23` writer - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ23_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_ERS_PEON` reader - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_PEON_R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_PEON` writer - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_PEON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_ERS_PEOFF` reader - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_PEOFF_R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_PEOFF` writer - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_PEOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RGRANT_DELAY_ERS_PEON` reader - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_PEON_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_PEON` writer - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_PEON_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_ERS_PEOFF` reader - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_PEOFF_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_PEOFF` writer - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_PEOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq01(&self) -> SCALE_ERS_SEQ01_R {
        SCALE_ERS_SEQ01_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq12(&self) -> SCALE_ERS_SEQ12_R {
        SCALE_ERS_SEQ12_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq23(&self) -> SCALE_ERS_SEQ23_R {
        SCALE_ERS_SEQ23_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_peon(&self) -> SCALE_ERS_PEON_R {
        SCALE_ERS_PEON_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_peoff(&self) -> SCALE_ERS_PEOFF_R {
        SCALE_ERS_PEOFF_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_peon(&self) -> RGRANT_DELAY_ERS_PEON_R {
        RGRANT_DELAY_ERS_PEON_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_peoff(&self) -> RGRANT_DELAY_ERS_PEOFF_R {
        RGRANT_DELAY_ERS_PEOFF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_seq01(&mut self) -> SCALE_ERS_SEQ01_W<RGRANT_SCALE_ERS_SPEC> {
        SCALE_ERS_SEQ01_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_seq12(&mut self) -> SCALE_ERS_SEQ12_W<RGRANT_SCALE_ERS_SPEC> {
        SCALE_ERS_SEQ12_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_seq23(&mut self) -> SCALE_ERS_SEQ23_W<RGRANT_SCALE_ERS_SPEC> {
        SCALE_ERS_SEQ23_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_peon(&mut self) -> SCALE_ERS_PEON_W<RGRANT_SCALE_ERS_SPEC> {
        SCALE_ERS_PEON_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_peoff(&mut self) -> SCALE_ERS_PEOFF_W<RGRANT_SCALE_ERS_SPEC> {
        SCALE_ERS_PEOFF_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_peon(&mut self) -> RGRANT_DELAY_ERS_PEON_W<RGRANT_SCALE_ERS_SPEC> {
        RGRANT_DELAY_ERS_PEON_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_peoff(&mut self) -> RGRANT_DELAY_ERS_PEOFF_W<RGRANT_SCALE_ERS_SPEC> {
        RGRANT_DELAY_ERS_PEOFF_W::new(self, 24)
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
#[doc = "R-grant delay scale for erase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgrant_scale_ers::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgrant_scale_ers::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGRANT_SCALE_ERS_SPEC;
impl crate::RegisterSpec for RGRANT_SCALE_ERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgrant_scale_ers::R`](R) reader structure"]
impl crate::Readable for RGRANT_SCALE_ERS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rgrant_scale_ers::W`](W) writer structure"]
impl crate::Writable for RGRANT_SCALE_ERS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGRANT_SCALE_ERS to value 0"]
impl crate::Resettable for RGRANT_SCALE_ERS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
