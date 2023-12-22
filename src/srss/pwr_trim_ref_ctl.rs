#[doc = "Register `PWR_TRIM_REF_CTL` reader"]
pub type R = crate::R<PWR_TRIM_REF_CTL_SPEC>;
#[doc = "Register `PWR_TRIM_REF_CTL` writer"]
pub type W = crate::W<PWR_TRIM_REF_CTL_SPEC>;
#[doc = "Field `ACT_REF_TCTRIM` reader - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_TCTRIM_R = crate::FieldReader;
#[doc = "Field `ACT_REF_TCTRIM` writer - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_TCTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACT_REF_ITRIM` reader - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_ITRIM_R = crate::FieldReader;
#[doc = "Field `ACT_REF_ITRIM` writer - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_ITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACT_REF_ABSTRIM` reader - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_ABSTRIM_R = crate::FieldReader;
#[doc = "Field `ACT_REF_ABSTRIM` writer - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_ABSTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ACT_REF_IBOOST` reader - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
pub type ACT_REF_IBOOST_R = crate::BitReader;
#[doc = "Field `ACT_REF_IBOOST` writer - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
pub type ACT_REF_IBOOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPSLP_REF_TCTRIM` reader - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type DPSLP_REF_TCTRIM_R = crate::FieldReader;
#[doc = "Field `DPSLP_REF_TCTRIM` writer - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type DPSLP_REF_TCTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPSLP_REF_ABSTRIM` reader - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DPSLP_REF_ABSTRIM_R = crate::FieldReader;
#[doc = "Field `DPSLP_REF_ABSTRIM` writer - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DPSLP_REF_ABSTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DPSLP_REF_ITRIM` reader - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DPSLP_REF_ITRIM_R = crate::FieldReader;
#[doc = "Field `DPSLP_REF_ITRIM` writer - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DPSLP_REF_ITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_tctrim(&self) -> ACT_REF_TCTRIM_R {
        ACT_REF_TCTRIM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_itrim(&self) -> ACT_REF_ITRIM_R {
        ACT_REF_ITRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_abstrim(&self) -> ACT_REF_ABSTRIM_R {
        ACT_REF_ABSTRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    pub fn act_ref_iboost(&self) -> ACT_REF_IBOOST_R {
        ACT_REF_IBOOST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn dpslp_ref_tctrim(&self) -> DPSLP_REF_TCTRIM_R {
        DPSLP_REF_TCTRIM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_abstrim(&self) -> DPSLP_REF_ABSTRIM_R {
        DPSLP_REF_ABSTRIM_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_itrim(&self) -> DPSLP_REF_ITRIM_R {
        DPSLP_REF_ITRIM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    #[must_use]
    pub fn act_ref_tctrim(&mut self) -> ACT_REF_TCTRIM_W<PWR_TRIM_REF_CTL_SPEC> {
        ACT_REF_TCTRIM_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    #[must_use]
    pub fn act_ref_itrim(&mut self) -> ACT_REF_ITRIM_W<PWR_TRIM_REF_CTL_SPEC> {
        ACT_REF_ITRIM_W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    #[must_use]
    pub fn act_ref_abstrim(&mut self) -> ACT_REF_ABSTRIM_W<PWR_TRIM_REF_CTL_SPEC> {
        ACT_REF_ABSTRIM_W::new(self, 8)
    }
    #[doc = "Bit 14 - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    #[must_use]
    pub fn act_ref_iboost(&mut self) -> ACT_REF_IBOOST_W<PWR_TRIM_REF_CTL_SPEC> {
        ACT_REF_IBOOST_W::new(self, 14)
    }
    #[doc = "Bits 16:19 - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    #[must_use]
    pub fn dpslp_ref_tctrim(&mut self) -> DPSLP_REF_TCTRIM_W<PWR_TRIM_REF_CTL_SPEC> {
        DPSLP_REF_TCTRIM_W::new(self, 16)
    }
    #[doc = "Bits 20:24 - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn dpslp_ref_abstrim(&mut self) -> DPSLP_REF_ABSTRIM_W<PWR_TRIM_REF_CTL_SPEC> {
        DPSLP_REF_ABSTRIM_W::new(self, 20)
    }
    #[doc = "Bits 28:31 - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn dpslp_ref_itrim(&mut self) -> DPSLP_REF_ITRIM_W<PWR_TRIM_REF_CTL_SPEC> {
        DPSLP_REF_ITRIM_W::new(self, 28)
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
#[doc = "Reference Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_trim_ref_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_trim_ref_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_TRIM_REF_CTL_SPEC;
impl crate::RegisterSpec for PWR_TRIM_REF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_trim_ref_ctl::R`](R) reader structure"]
impl crate::Readable for PWR_TRIM_REF_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_trim_ref_ctl::W`](W) writer structure"]
impl crate::Writable for PWR_TRIM_REF_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_TRIM_REF_CTL to value 0x70f0_0000"]
impl crate::Resettable for PWR_TRIM_REF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x70f0_0000;
}
