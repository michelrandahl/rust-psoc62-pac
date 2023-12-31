#[doc = "Register `CLK_TRIM_ILO_CTL` reader"]
pub type R = crate::R<CLK_TRIM_ILO_CTL_SPEC>;
#[doc = "Register `CLK_TRIM_ILO_CTL` writer"]
pub type W = crate::W<CLK_TRIM_ILO_CTL_SPEC>;
#[doc = "Field `ILO_FTRIM` reader - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
pub type ILO_FTRIM_R = crate::FieldReader;
#[doc = "Field `ILO_FTRIM` writer - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
pub type ILO_FTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    pub fn ilo_ftrim(&self) -> ILO_FTRIM_R {
        ILO_FTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    #[must_use]
    pub fn ilo_ftrim(&mut self) -> ILO_FTRIM_W<CLK_TRIM_ILO_CTL_SPEC> {
        ILO_FTRIM_W::new(self, 0)
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
#[doc = "ILO Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_ilo_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_ilo_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_TRIM_ILO_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_ILO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_ilo_ctl::R`](R) reader structure"]
impl crate::Readable for CLK_TRIM_ILO_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_ilo_ctl::W`](W) writer structure"]
impl crate::Writable for CLK_TRIM_ILO_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_ILO_CTL to value 0x2c"]
impl crate::Resettable for CLK_TRIM_ILO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2c;
}
