#[doc = "Register `CLK_FLL_CONFIG2` reader"]
pub type R = crate::R<CLK_FLL_CONFIG2_SPEC>;
#[doc = "Register `CLK_FLL_CONFIG2` writer"]
pub type W = crate::W<CLK_FLL_CONFIG2_SPEC>;
#[doc = "Field `FLL_REF_DIV` reader - Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
pub type FLL_REF_DIV_R = crate::FieldReader<u16>;
#[doc = "Field `FLL_REF_DIV` writer - Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
pub type FLL_REF_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `LOCK_TOL` reader - Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
pub type LOCK_TOL_R = crate::FieldReader<u16>;
#[doc = "Field `LOCK_TOL` writer - Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
pub type LOCK_TOL_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:12 - Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
    #[inline(always)]
    pub fn fll_ref_div(&self) -> FLL_REF_DIV_R {
        FLL_REF_DIV_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:24 - Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
    #[inline(always)]
    pub fn lock_tol(&self) -> LOCK_TOL_R {
        LOCK_TOL_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
    #[inline(always)]
    #[must_use]
    pub fn fll_ref_div(&mut self) -> FLL_REF_DIV_W<CLK_FLL_CONFIG2_SPEC> {
        FLL_REF_DIV_W::new(self, 0)
    }
    #[doc = "Bits 16:24 - Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
    #[inline(always)]
    #[must_use]
    pub fn lock_tol(&mut self) -> LOCK_TOL_W<CLK_FLL_CONFIG2_SPEC> {
        LOCK_TOL_W::new(self, 16)
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
#[doc = "FLL Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_fll_config2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_fll_config2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_FLL_CONFIG2_SPEC;
impl crate::RegisterSpec for CLK_FLL_CONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_fll_config2::R`](R) reader structure"]
impl crate::Readable for CLK_FLL_CONFIG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_fll_config2::W`](W) writer structure"]
impl crate::Writable for CLK_FLL_CONFIG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_FLL_CONFIG2 to value 0x0002_0001"]
impl crate::Resettable for CLK_FLL_CONFIG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0001;
}
