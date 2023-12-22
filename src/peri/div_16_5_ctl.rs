#[doc = "Register `DIV_16_5_CTL[%s]` reader"]
pub type R = crate::R<DIV_16_5_CTL_SPEC>;
#[doc = "Register `DIV_16_5_CTL[%s]` writer"]
pub type W = crate::W<DIV_16_5_CTL_SPEC>;
#[doc = "Field `EN` reader - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
pub type EN_R = crate::BitReader;
#[doc = "Field `FRAC5_DIV` reader - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type FRAC5_DIV_R = crate::FieldReader;
#[doc = "Field `FRAC5_DIV` writer - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type FRAC5_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INT16_DIV` reader - Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 65,536 31/32\\]
in 1/32 increments. For the generation of a divided clock, the division range is restricted to \\[2, 65,536 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 65,536\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type INT16_DIV_R = crate::FieldReader<u16>;
#[doc = "Field `INT16_DIV` writer - Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 65,536 31/32\\]
in 1/32 increments. For the generation of a divided clock, the division range is restricted to \\[2, 65,536 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 65,536\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type INT16_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:7 - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn frac5_div(&self) -> FRAC5_DIV_R {
        FRAC5_DIV_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:23 - Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 65,536 31/32\\]
in 1/32 increments. For the generation of a divided clock, the division range is restricted to \\[2, 65,536 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 65,536\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int16_div(&self) -> INT16_DIV_R {
        INT16_DIV_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:7 - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn frac5_div(&mut self) -> FRAC5_DIV_W<DIV_16_5_CTL_SPEC> {
        FRAC5_DIV_W::new(self, 3)
    }
    #[doc = "Bits 8:23 - Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 65,536 31/32\\]
in 1/32 increments. For the generation of a divided clock, the division range is restricted to \\[2, 65,536 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 65,536\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn int16_div(&mut self) -> INT16_DIV_W<DIV_16_5_CTL_SPEC> {
        INT16_DIV_W::new(self, 8)
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
#[doc = "Divider control (for 16.5 divider)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div_16_5_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_16_5_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_16_5_CTL_SPEC;
impl crate::RegisterSpec for DIV_16_5_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_16_5_ctl::R`](R) reader structure"]
impl crate::Readable for DIV_16_5_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div_16_5_ctl::W`](W) writer structure"]
impl crate::Writable for DIV_16_5_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV_16_5_CTL[%s]
to value 0"]
impl crate::Resettable for DIV_16_5_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
