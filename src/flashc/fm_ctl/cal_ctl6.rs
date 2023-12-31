#[doc = "Register `CAL_CTL6` reader"]
pub type R = crate::R<CAL_CTL6_SPEC>;
#[doc = "Register `CAL_CTL6` writer"]
pub type W = crate::W<CAL_CTL6_SPEC>;
#[doc = "Field `SA_CTL_TRIM_T1_ULP_HV` reader - clk_trk delay"]
pub type SA_CTL_TRIM_T1_ULP_HV_R = crate::BitReader;
#[doc = "Field `SA_CTL_TRIM_T1_ULP_HV` writer - clk_trk delay"]
pub type SA_CTL_TRIM_T1_ULP_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA_CTL_TRIM_T4_ULP_HV` reader - SA_CTL_TRIM_T4_ULP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV&lt;1:0> = eqc (eq cap trim)"]
pub type SA_CTL_TRIM_T4_ULP_HV_R = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T4_ULP_HV` writer - SA_CTL_TRIM_T4_ULP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV&lt;1:0> = eqc (eq cap trim)"]
pub type SA_CTL_TRIM_T4_ULP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SA_CTL_TRIM_T5_ULP_HV` reader - SA_CTL_TRIM_T5_ULP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV&lt;1:0> = evc (integration cap trim)"]
pub type SA_CTL_TRIM_T5_ULP_HV_R = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T5_ULP_HV` writer - SA_CTL_TRIM_T5_ULP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV&lt;1:0> = evc (integration cap trim)"]
pub type SA_CTL_TRIM_T5_ULP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SA_CTL_TRIM_T6_ULP_HV` reader - SA_CTL_TRIM_T6_ULP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV&lt;0> = ecn (enable cap trim)"]
pub type SA_CTL_TRIM_T6_ULP_HV_R = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T6_ULP_HV` writer - SA_CTL_TRIM_T6_ULP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV&lt;0> = ecn (enable cap trim)"]
pub type SA_CTL_TRIM_T6_ULP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SA_CTL_TRIM_T8_ULP_HV` reader - saen3 pulse width trim (Current trim)"]
pub type SA_CTL_TRIM_T8_ULP_HV_R = crate::BitReader;
#[doc = "Field `SA_CTL_TRIM_T8_ULP_HV` writer - saen3 pulse width trim (Current trim)"]
pub type SA_CTL_TRIM_T8_ULP_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA_CTL_TRIM_T1_LP_HV` reader - clk_trk delay"]
pub type SA_CTL_TRIM_T1_LP_HV_R = crate::BitReader;
#[doc = "Field `SA_CTL_TRIM_T1_LP_HV` writer - clk_trk delay"]
pub type SA_CTL_TRIM_T1_LP_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA_CTL_TRIM_T4_LP_HV` reader - SA_CTL_TRIM_T4_LP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV&lt;1:0> = eqc (eq cap trim)"]
pub type SA_CTL_TRIM_T4_LP_HV_R = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T4_LP_HV` writer - SA_CTL_TRIM_T4_LP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV&lt;1:0> = eqc (eq cap trim)"]
pub type SA_CTL_TRIM_T4_LP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SA_CTL_TRIM_T5_LP_HV` reader - SA_CTL_TRIM_T5_LP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV&lt;1:0> = evc (integration cap trim)"]
pub type SA_CTL_TRIM_T5_LP_HV_R = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T5_LP_HV` writer - SA_CTL_TRIM_T5_LP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV&lt;1:0> = evc (integration cap trim)"]
pub type SA_CTL_TRIM_T5_LP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SA_CTL_TRIM_T6_LP_HV` reader - SA_CTL_TRIM_T6_LP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV&lt;0> = ecn (enable cap trim)"]
pub type SA_CTL_TRIM_T6_LP_HV_R = crate::FieldReader;
#[doc = "Field `SA_CTL_TRIM_T6_LP_HV` writer - SA_CTL_TRIM_T6_LP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV&lt;0> = ecn (enable cap trim)"]
pub type SA_CTL_TRIM_T6_LP_HV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SA_CTL_TRIM_T8_LP_HV` reader - saen3 pulse width trim (Current trim)"]
pub type SA_CTL_TRIM_T8_LP_HV_R = crate::BitReader;
#[doc = "Field `SA_CTL_TRIM_T8_LP_HV` writer - saen3 pulse width trim (Current trim)"]
pub type SA_CTL_TRIM_T8_LP_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - clk_trk delay"]
    #[inline(always)]
    pub fn sa_ctl_trim_t1_ulp_hv(&self) -> SA_CTL_TRIM_T1_ULP_HV_R {
        SA_CTL_TRIM_T1_ULP_HV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SA_CTL_TRIM_T4_ULP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV&lt;1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t4_ulp_hv(&self) -> SA_CTL_TRIM_T4_ULP_HV_R {
        SA_CTL_TRIM_T4_ULP_HV_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - SA_CTL_TRIM_T5_ULP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV&lt;1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t5_ulp_hv(&self) -> SA_CTL_TRIM_T5_ULP_HV_R {
        SA_CTL_TRIM_T5_ULP_HV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - SA_CTL_TRIM_T6_ULP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV&lt;0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t6_ulp_hv(&self) -> SA_CTL_TRIM_T6_ULP_HV_R {
        SA_CTL_TRIM_T6_ULP_HV_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t8_ulp_hv(&self) -> SA_CTL_TRIM_T8_ULP_HV_R {
        SA_CTL_TRIM_T8_ULP_HV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - clk_trk delay"]
    #[inline(always)]
    pub fn sa_ctl_trim_t1_lp_hv(&self) -> SA_CTL_TRIM_T1_LP_HV_R {
        SA_CTL_TRIM_T1_LP_HV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - SA_CTL_TRIM_T4_LP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV&lt;1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t4_lp_hv(&self) -> SA_CTL_TRIM_T4_LP_HV_R {
        SA_CTL_TRIM_T4_LP_HV_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - SA_CTL_TRIM_T5_LP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV&lt;1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t5_lp_hv(&self) -> SA_CTL_TRIM_T5_LP_HV_R {
        SA_CTL_TRIM_T5_LP_HV_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:18 - SA_CTL_TRIM_T6_LP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV&lt;0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t6_lp_hv(&self) -> SA_CTL_TRIM_T6_LP_HV_R {
        SA_CTL_TRIM_T6_LP_HV_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t8_lp_hv(&self) -> SA_CTL_TRIM_T8_LP_HV_R {
        SA_CTL_TRIM_T8_LP_HV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_trk delay"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t1_ulp_hv(&mut self) -> SA_CTL_TRIM_T1_ULP_HV_W<CAL_CTL6_SPEC> {
        SA_CTL_TRIM_T1_ULP_HV_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - SA_CTL_TRIM_T4_ULP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV&lt;1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t4_ulp_hv(&mut self) -> SA_CTL_TRIM_T4_ULP_HV_W<CAL_CTL6_SPEC> {
        SA_CTL_TRIM_T4_ULP_HV_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - SA_CTL_TRIM_T5_ULP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV&lt;1:0> = evc (integration cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t5_ulp_hv(&mut self) -> SA_CTL_TRIM_T5_ULP_HV_W<CAL_CTL6_SPEC> {
        SA_CTL_TRIM_T5_ULP_HV_W::new(self, 4)
    }
    #[doc = "Bits 7:8 - SA_CTL_TRIM_T6_ULP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV&lt;0> = ecn (enable cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t6_ulp_hv(&mut self) -> SA_CTL_TRIM_T6_ULP_HV_W<CAL_CTL6_SPEC> {
        SA_CTL_TRIM_T6_ULP_HV_W::new(self, 7)
    }
    #[doc = "Bit 9 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t8_ulp_hv(&mut self) -> SA_CTL_TRIM_T8_ULP_HV_W<CAL_CTL6_SPEC> {
        SA_CTL_TRIM_T8_ULP_HV_W::new(self, 9)
    }
    #[doc = "Bit 10 - clk_trk delay"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t1_lp_hv(&mut self) -> SA_CTL_TRIM_T1_LP_HV_W<CAL_CTL6_SPEC> {
        SA_CTL_TRIM_T1_LP_HV_W::new(self, 10)
    }
    #[doc = "Bits 11:13 - SA_CTL_TRIM_T4_LP_HV&lt;2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV&lt;1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t4_lp_hv(&mut self) -> SA_CTL_TRIM_T4_LP_HV_W<CAL_CTL6_SPEC> {
        SA_CTL_TRIM_T4_LP_HV_W::new(self, 11)
    }
    #[doc = "Bits 14:16 - SA_CTL_TRIM_T5_LP_HV&lt;2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV&lt;1:0> = evc (integration cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t5_lp_hv(&mut self) -> SA_CTL_TRIM_T5_LP_HV_W<CAL_CTL6_SPEC> {
        SA_CTL_TRIM_T5_LP_HV_W::new(self, 14)
    }
    #[doc = "Bits 17:18 - SA_CTL_TRIM_T6_LP_HV&lt;1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV&lt;0> = ecn (enable cap trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t6_lp_hv(&mut self) -> SA_CTL_TRIM_T6_LP_HV_W<CAL_CTL6_SPEC> {
        SA_CTL_TRIM_T6_LP_HV_W::new(self, 17)
    }
    #[doc = "Bit 19 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    #[must_use]
    pub fn sa_ctl_trim_t8_lp_hv(&mut self) -> SA_CTL_TRIM_T8_LP_HV_W<CAL_CTL6_SPEC> {
        SA_CTL_TRIM_T8_LP_HV_W::new(self, 19)
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
#[doc = "SA trim LP/ULP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_CTL6_SPEC;
impl crate::RegisterSpec for CAL_CTL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl6::R`](R) reader structure"]
impl crate::Readable for CAL_CTL6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl6::W`](W) writer structure"]
impl crate::Writable for CAL_CTL6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_CTL6 to value 0x0003_6f7f"]
impl crate::Resettable for CAL_CTL6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_6f7f;
}
