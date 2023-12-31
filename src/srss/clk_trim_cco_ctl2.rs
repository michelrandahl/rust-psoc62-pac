#[doc = "Register `CLK_TRIM_CCO_CTL2` reader"]
pub type R = crate::R<CLK_TRIM_CCO_CTL2_SPEC>;
#[doc = "Register `CLK_TRIM_CCO_CTL2` writer"]
pub type W = crate::W<CLK_TRIM_CCO_CTL2_SPEC>;
#[doc = "Field `CCO_FCTRIM1` reader - CCO frequency 1st range calibration"]
pub type CCO_FCTRIM1_R = crate::FieldReader;
#[doc = "Field `CCO_FCTRIM1` writer - CCO frequency 1st range calibration"]
pub type CCO_FCTRIM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCO_FCTRIM2` reader - CCO frequency 2nd range calibration"]
pub type CCO_FCTRIM2_R = crate::FieldReader;
#[doc = "Field `CCO_FCTRIM2` writer - CCO frequency 2nd range calibration"]
pub type CCO_FCTRIM2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCO_FCTRIM3` reader - CCO frequency 3rd range calibration"]
pub type CCO_FCTRIM3_R = crate::FieldReader;
#[doc = "Field `CCO_FCTRIM3` writer - CCO frequency 3rd range calibration"]
pub type CCO_FCTRIM3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCO_FCTRIM4` reader - CCO frequency 4th range calibration"]
pub type CCO_FCTRIM4_R = crate::FieldReader;
#[doc = "Field `CCO_FCTRIM4` writer - CCO frequency 4th range calibration"]
pub type CCO_FCTRIM4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCO_FCTRIM5` reader - CCO frequency 5th range calibration"]
pub type CCO_FCTRIM5_R = crate::FieldReader;
#[doc = "Field `CCO_FCTRIM5` writer - CCO frequency 5th range calibration"]
pub type CCO_FCTRIM5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CCO frequency 1st range calibration"]
    #[inline(always)]
    pub fn cco_fctrim1(&self) -> CCO_FCTRIM1_R {
        CCO_FCTRIM1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - CCO frequency 2nd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim2(&self) -> CCO_FCTRIM2_R {
        CCO_FCTRIM2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - CCO frequency 3rd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim3(&self) -> CCO_FCTRIM3_R {
        CCO_FCTRIM3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - CCO frequency 4th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim4(&self) -> CCO_FCTRIM4_R {
        CCO_FCTRIM4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - CCO frequency 5th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim5(&self) -> CCO_FCTRIM5_R {
        CCO_FCTRIM5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - CCO frequency 1st range calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cco_fctrim1(&mut self) -> CCO_FCTRIM1_W<CLK_TRIM_CCO_CTL2_SPEC> {
        CCO_FCTRIM1_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - CCO frequency 2nd range calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cco_fctrim2(&mut self) -> CCO_FCTRIM2_W<CLK_TRIM_CCO_CTL2_SPEC> {
        CCO_FCTRIM2_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - CCO frequency 3rd range calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cco_fctrim3(&mut self) -> CCO_FCTRIM3_W<CLK_TRIM_CCO_CTL2_SPEC> {
        CCO_FCTRIM3_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - CCO frequency 4th range calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cco_fctrim4(&mut self) -> CCO_FCTRIM4_W<CLK_TRIM_CCO_CTL2_SPEC> {
        CCO_FCTRIM4_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - CCO frequency 5th range calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cco_fctrim5(&mut self) -> CCO_FCTRIM5_W<CLK_TRIM_CCO_CTL2_SPEC> {
        CCO_FCTRIM5_W::new(self, 20)
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
#[doc = "CCO Trim Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_cco_ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_cco_ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_TRIM_CCO_CTL2_SPEC;
impl crate::RegisterSpec for CLK_TRIM_CCO_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_cco_ctl2::R`](R) reader structure"]
impl crate::Readable for CLK_TRIM_CCO_CTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_cco_ctl2::W`](W) writer structure"]
impl crate::Writable for CLK_TRIM_CCO_CTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_CCO_CTL2 to value 0x0088_4110"]
impl crate::Resettable for CLK_TRIM_CCO_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0088_4110;
}
