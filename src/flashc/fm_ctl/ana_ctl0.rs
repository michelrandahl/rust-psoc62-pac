#[doc = "Register `ANA_CTL0` reader"]
pub type R = crate::R<ANA_CTL0_SPEC>;
#[doc = "Register `ANA_CTL0` writer"]
pub type W = crate::W<ANA_CTL0_SPEC>;
#[doc = "Field `MDAC` reader - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
pub type MDAC_R = crate::FieldReader;
#[doc = "Field `MDAC` writer - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
pub type MDAC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSLDAC` reader - Trimming of common source line DAC."]
pub type CSLDAC_R = crate::FieldReader;
#[doc = "Field `CSLDAC` writer - Trimming of common source line DAC."]
pub type CSLDAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLIP_AMUXBUS_AB` reader - Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
pub type FLIP_AMUXBUS_AB_R = crate::BitReader;
#[doc = "Field `FLIP_AMUXBUS_AB` writer - Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
pub type FLIP_AMUXBUS_AB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDAC_MIN` reader - NDAC staircase min value"]
pub type NDAC_MIN_R = crate::FieldReader;
#[doc = "Field `NDAC_MIN` writer - NDAC staircase min value"]
pub type NDAC_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PDAC_MIN` reader - PDAC staircase min value"]
pub type PDAC_MIN_R = crate::FieldReader;
#[doc = "Field `PDAC_MIN` writer - PDAC staircase min value"]
pub type PDAC_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCALE_PRG_SEQ01` reader - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_PRG_SEQ01_R = crate::FieldReader;
#[doc = "Field `SCALE_PRG_SEQ01` writer - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_PRG_SEQ01_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_PRG_SEQ12` reader - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_PRG_SEQ12_R = crate::FieldReader;
#[doc = "Field `SCALE_PRG_SEQ12` writer - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_PRG_SEQ12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_PRG_SEQ23` reader - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_PRG_SEQ23_R = crate::FieldReader;
#[doc = "Field `SCALE_PRG_SEQ23` writer - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_PRG_SEQ23_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_SEQ30` reader - PROG&amp;PRE_PROG&amp; ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_SEQ30_R = crate::FieldReader;
#[doc = "Field `SCALE_SEQ30` writer - PROG&amp;PRE_PROG&amp; ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_SEQ30_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_PRG_PEON` reader - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_PRG_PEON_R = crate::FieldReader;
#[doc = "Field `SCALE_PRG_PEON` writer - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_PRG_PEON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_PRG_PEOFF` reader - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_PRG_PEOFF_R = crate::FieldReader;
#[doc = "Field `SCALE_PRG_PEOFF` writer - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_PRG_PEOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn mdac(&self) -> MDAC_R {
        MDAC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    pub fn csldac(&self) -> CSLDAC_R {
        CSLDAC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn flip_amuxbus_ab(&self) -> FLIP_AMUXBUS_AB_R {
        FLIP_AMUXBUS_AB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - NDAC staircase min value"]
    #[inline(always)]
    pub fn ndac_min(&self) -> NDAC_MIN_R {
        NDAC_MIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PDAC staircase min value"]
    #[inline(always)]
    pub fn pdac_min(&self) -> PDAC_MIN_R {
        PDAC_MIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq01(&self) -> SCALE_PRG_SEQ01_R {
        SCALE_PRG_SEQ01_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq12(&self) -> SCALE_PRG_SEQ12_R {
        SCALE_PRG_SEQ12_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq23(&self) -> SCALE_PRG_SEQ23_R {
        SCALE_PRG_SEQ23_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PROG&amp;PRE_PROG&amp; ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_seq30(&self) -> SCALE_SEQ30_R {
        SCALE_SEQ30_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_peon(&self) -> SCALE_PRG_PEON_R {
        SCALE_PRG_PEON_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_peoff(&self) -> SCALE_PRG_PEOFF_R {
        SCALE_PRG_PEOFF_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    #[must_use]
    pub fn mdac(&mut self) -> MDAC_W<ANA_CTL0_SPEC> {
        MDAC_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    #[must_use]
    pub fn csldac(&mut self) -> CSLDAC_W<ANA_CTL0_SPEC> {
        CSLDAC_W::new(self, 8)
    }
    #[doc = "Bit 11 - Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
    #[inline(always)]
    #[must_use]
    pub fn flip_amuxbus_ab(&mut self) -> FLIP_AMUXBUS_AB_W<ANA_CTL0_SPEC> {
        FLIP_AMUXBUS_AB_W::new(self, 11)
    }
    #[doc = "Bits 12:15 - NDAC staircase min value"]
    #[inline(always)]
    #[must_use]
    pub fn ndac_min(&mut self) -> NDAC_MIN_W<ANA_CTL0_SPEC> {
        NDAC_MIN_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - PDAC staircase min value"]
    #[inline(always)]
    #[must_use]
    pub fn pdac_min(&mut self) -> PDAC_MIN_W<ANA_CTL0_SPEC> {
        PDAC_MIN_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_prg_seq01(&mut self) -> SCALE_PRG_SEQ01_W<ANA_CTL0_SPEC> {
        SCALE_PRG_SEQ01_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_prg_seq12(&mut self) -> SCALE_PRG_SEQ12_W<ANA_CTL0_SPEC> {
        SCALE_PRG_SEQ12_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_prg_seq23(&mut self) -> SCALE_PRG_SEQ23_W<ANA_CTL0_SPEC> {
        SCALE_PRG_SEQ23_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - PROG&amp;PRE_PROG&amp; ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_seq30(&mut self) -> SCALE_SEQ30_W<ANA_CTL0_SPEC> {
        SCALE_SEQ30_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_prg_peon(&mut self) -> SCALE_PRG_PEON_W<ANA_CTL0_SPEC> {
        SCALE_PRG_PEON_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_prg_peoff(&mut self) -> SCALE_PRG_PEOFF_W<ANA_CTL0_SPEC> {
        SCALE_PRG_PEOFF_W::new(self, 30)
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
#[doc = "Analog control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_CTL0_SPEC;
impl crate::RegisterSpec for ANA_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_ctl0::R`](R) reader structure"]
impl crate::Readable for ANA_CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_ctl0::W`](W) writer structure"]
impl crate::Writable for ANA_CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_CTL0 to value 0x0400"]
impl crate::Resettable for ANA_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
