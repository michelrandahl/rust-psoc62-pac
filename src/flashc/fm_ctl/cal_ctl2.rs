#[doc = "Register `CAL_CTL2` reader"]
pub type R = crate::R<CAL_CTL2_SPEC>;
#[doc = "Register `CAL_CTL2` writer"]
pub type W = crate::W<CAL_CTL2_SPEC>;
#[doc = "Field `ICREF_TRIM_LO_HV` reader - LO Bandgap Current trim control."]
pub type ICREF_TRIM_LO_HV_R = crate::FieldReader;
#[doc = "Field `ICREF_TRIM_LO_HV` writer - LO Bandgap Current trim control."]
pub type ICREF_TRIM_LO_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ICREF_TRIM_HI_HV` reader - HI Bandgap Current trim control."]
pub type ICREF_TRIM_HI_HV_R = crate::FieldReader;
#[doc = "Field `ICREF_TRIM_HI_HV` writer - HI Bandgap Current trim control."]
pub type ICREF_TRIM_HI_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IPREF_TRIM_LO_HV` reader - LO Bandgap IPTAT trim control."]
pub type IPREF_TRIM_LO_HV_R = crate::FieldReader;
#[doc = "Field `IPREF_TRIM_LO_HV` writer - LO Bandgap IPTAT trim control."]
pub type IPREF_TRIM_LO_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IPREF_TRIM_HI_HV` reader - HI Bandgap IPTAT trim control."]
pub type IPREF_TRIM_HI_HV_R = crate::FieldReader;
#[doc = "Field `IPREF_TRIM_HI_HV` writer - HI Bandgap IPTAT trim control."]
pub type IPREF_TRIM_HI_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_lo_hv(&self) -> ICREF_TRIM_LO_HV_R {
        ICREF_TRIM_LO_HV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_hi_hv(&self) -> ICREF_TRIM_HI_HV_R {
        ICREF_TRIM_HI_HV_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - LO Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_lo_hv(&self) -> IPREF_TRIM_LO_HV_R {
        IPREF_TRIM_LO_HV_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_hi_hv(&self) -> IPREF_TRIM_HI_HV_R {
        IPREF_TRIM_HI_HV_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    #[must_use]
    pub fn icref_trim_lo_hv(&mut self) -> ICREF_TRIM_LO_HV_W<CAL_CTL2_SPEC> {
        ICREF_TRIM_LO_HV_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - HI Bandgap Current trim control."]
    #[inline(always)]
    #[must_use]
    pub fn icref_trim_hi_hv(&mut self) -> ICREF_TRIM_HI_HV_W<CAL_CTL2_SPEC> {
        ICREF_TRIM_HI_HV_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - LO Bandgap IPTAT trim control."]
    #[inline(always)]
    #[must_use]
    pub fn ipref_trim_lo_hv(&mut self) -> IPREF_TRIM_LO_HV_W<CAL_CTL2_SPEC> {
        IPREF_TRIM_LO_HV_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - HI Bandgap IPTAT trim control."]
    #[inline(always)]
    #[must_use]
    pub fn ipref_trim_hi_hv(&mut self) -> IPREF_TRIM_HI_HV_W<CAL_CTL2_SPEC> {
        IPREF_TRIM_HI_HV_W::new(self, 15)
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
#[doc = "Cal control BG LO&amp;HI trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_CTL2_SPEC;
impl crate::RegisterSpec for CAL_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl2::R`](R) reader structure"]
impl crate::Readable for CAL_CTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl2::W`](W) writer structure"]
impl crate::Writable for CAL_CTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_CTL2 to value 0x0007_be10"]
impl crate::Resettable for CAL_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_be10;
}
