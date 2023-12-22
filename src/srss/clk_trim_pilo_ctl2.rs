#[doc = "Register `CLK_TRIM_PILO_CTL2` reader"]
pub type R = crate::R<CLK_TRIM_PILO_CTL2_SPEC>;
#[doc = "Register `CLK_TRIM_PILO_CTL2` writer"]
pub type W = crate::W<CLK_TRIM_PILO_CTL2_SPEC>;
#[doc = "Field `PILO_VREF_TRIM` reader - Trim for voltage reference"]
pub type PILO_VREF_TRIM_R = crate::FieldReader;
#[doc = "Field `PILO_VREF_TRIM` writer - Trim for voltage reference"]
pub type PILO_VREF_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PILO_IREFBM_TRIM` reader - Trim for beta-multiplier current reference"]
pub type PILO_IREFBM_TRIM_R = crate::FieldReader;
#[doc = "Field `PILO_IREFBM_TRIM` writer - Trim for beta-multiplier current reference"]
pub type PILO_IREFBM_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PILO_IREF_TRIM` reader - Trim for current reference"]
pub type PILO_IREF_TRIM_R = crate::FieldReader;
#[doc = "Field `PILO_IREF_TRIM` writer - Trim for current reference"]
pub type PILO_IREF_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Trim for voltage reference"]
    #[inline(always)]
    pub fn pilo_vref_trim(&self) -> PILO_VREF_TRIM_R {
        PILO_VREF_TRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Trim for beta-multiplier current reference"]
    #[inline(always)]
    pub fn pilo_irefbm_trim(&self) -> PILO_IREFBM_TRIM_R {
        PILO_IREFBM_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Trim for current reference"]
    #[inline(always)]
    pub fn pilo_iref_trim(&self) -> PILO_IREF_TRIM_R {
        PILO_IREF_TRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trim for voltage reference"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_vref_trim(&mut self) -> PILO_VREF_TRIM_W<CLK_TRIM_PILO_CTL2_SPEC> {
        PILO_VREF_TRIM_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Trim for beta-multiplier current reference"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_irefbm_trim(&mut self) -> PILO_IREFBM_TRIM_W<CLK_TRIM_PILO_CTL2_SPEC> {
        PILO_IREFBM_TRIM_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Trim for current reference"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_iref_trim(&mut self) -> PILO_IREF_TRIM_W<CLK_TRIM_PILO_CTL2_SPEC> {
        PILO_IREF_TRIM_W::new(self, 16)
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
#[doc = "PILO Trim Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_pilo_ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_TRIM_PILO_CTL2_SPEC;
impl crate::RegisterSpec for CLK_TRIM_PILO_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_pilo_ctl2::R`](R) reader structure"]
impl crate::Readable for CLK_TRIM_PILO_CTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_pilo_ctl2::W`](W) writer structure"]
impl crate::Writable for CLK_TRIM_PILO_CTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_PILO_CTL2 to value 0x00da_10e0"]
impl crate::Resettable for CLK_TRIM_PILO_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x00da_10e0;
}
