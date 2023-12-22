#[doc = "Register `DIV_8_CTL[%s]` reader"]
pub type R = crate::R<DIV_8_CTL_SPEC>;
#[doc = "Register `DIV_8_CTL[%s]` writer"]
pub type W = crate::W<DIV_8_CTL_SPEC>;
#[doc = "Field `EN` reader - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
pub type EN_R = crate::BitReader;
#[doc = "Field `INT8_DIV` reader - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is restricted to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type INT8_DIV_R = crate::FieldReader;
#[doc = "Field `INT8_DIV` writer - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is restricted to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type INT8_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is restricted to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int8_div(&self) -> INT8_DIV_R {
        INT8_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is restricted to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn int8_div(&mut self) -> INT8_DIV_W<DIV_8_CTL_SPEC> {
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
#[doc = "Divider control (for 8.0 divider)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div_8_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_8_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_8_CTL_SPEC;
impl crate::RegisterSpec for DIV_8_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_8_ctl::R`](R) reader structure"]
impl crate::Readable for DIV_8_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div_8_ctl::W`](W) writer structure"]
impl crate::Writable for DIV_8_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV_8_CTL[%s]
to value 0"]
impl crate::Resettable for DIV_8_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
