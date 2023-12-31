#[doc = "Register `CM4_CLOCK_CTL` reader"]
pub type R = crate::R<CM4_CLOCK_CTL_SPEC>;
#[doc = "Register `CM4_CLOCK_CTL` writer"]
pub type W = crate::W<CM4_CLOCK_CTL_SPEC>;
#[doc = "Field `FAST_INT_DIV` reader - Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type FAST_INT_DIV_R = crate::FieldReader;
#[doc = "Field `FAST_INT_DIV` writer - Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type FAST_INT_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn fast_int_div(&self) -> FAST_INT_DIV_R {
        FAST_INT_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn fast_int_div(&mut self) -> FAST_INT_DIV_W<CM4_CLOCK_CTL_SPEC> {
        FAST_INT_DIV_W::new(self, 8)
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
#[doc = "CM4 clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_clock_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_clock_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM4_CLOCK_CTL_SPEC;
impl crate::RegisterSpec for CM4_CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_clock_ctl::R`](R) reader structure"]
impl crate::Readable for CM4_CLOCK_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm4_clock_ctl::W`](W) writer structure"]
impl crate::Writable for CM4_CLOCK_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM4_CLOCK_CTL to value 0"]
impl crate::Resettable for CM4_CLOCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
