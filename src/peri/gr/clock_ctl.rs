#[doc = "Register `CLOCK_CTL` reader"]
pub type R = crate::R<CLOCK_CTL_SPEC>;
#[doc = "Register `CLOCK_CTL` writer"]
pub type W = crate::W<CLOCK_CTL_SPEC>;
#[doc = "Field `INT8_DIV` reader - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type INT8_DIV_R = crate::FieldReader;
#[doc = "Field `INT8_DIV` writer - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type INT8_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int8_div(&self) -> INT8_DIV_R {
        INT8_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn int8_div(&mut self) -> INT8_DIV_W<CLOCK_CTL_SPEC> {
        INT8_DIV_W::new(self, 8)
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
#[doc = "Clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_CTL_SPEC;
impl crate::RegisterSpec for CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_ctl::R`](R) reader structure"]
impl crate::Readable for CLOCK_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_ctl::W`](W) writer structure"]
impl crate::Writable for CLOCK_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK_CTL to value 0"]
impl crate::Resettable for CLOCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
