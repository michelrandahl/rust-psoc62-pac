#[doc = "Register `CLOCK_CTL[%s]` reader"]
pub type R = crate::R<CLOCK_CTL_SPEC>;
#[doc = "Register `CLOCK_CTL[%s]` writer"]
pub type W = crate::W<CLOCK_CTL_SPEC>;
#[doc = "Field `DIV_SEL` reader - Specifies one of the dividers of the divider type specified by TYPE_SEL. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out-of-phase dividers, spurious clock control signals may be generated for one 'clk_peri' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (DIV_SEL is '255' and TYPE_SEL is '3') for a transition time that is larger than the smaller of the two divider periods."]
pub type DIV_SEL_R = crate::FieldReader;
#[doc = "Field `DIV_SEL` writer - Specifies one of the dividers of the divider type specified by TYPE_SEL. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out-of-phase dividers, spurious clock control signals may be generated for one 'clk_peri' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (DIV_SEL is '255' and TYPE_SEL is '3') for a transition time that is larger than the smaller of the two divider periods."]
pub type DIV_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TYPE_SEL` reader - Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub type TYPE_SEL_R = crate::FieldReader;
#[doc = "Field `TYPE_SEL` writer - Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub type TYPE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Specifies one of the dividers of the divider type specified by TYPE_SEL. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out-of-phase dividers, spurious clock control signals may be generated for one 'clk_peri' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (DIV_SEL is '255' and TYPE_SEL is '3') for a transition time that is larger than the smaller of the two divider periods."]
    #[inline(always)]
    pub fn div_sel(&self) -> DIV_SEL_R {
        DIV_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn type_sel(&self) -> TYPE_SEL_R {
        TYPE_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies one of the dividers of the divider type specified by TYPE_SEL. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out-of-phase dividers, spurious clock control signals may be generated for one 'clk_peri' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (DIV_SEL is '255' and TYPE_SEL is '3') for a transition time that is larger than the smaller of the two divider periods."]
    #[inline(always)]
    #[must_use]
    pub fn div_sel(&mut self) -> DIV_SEL_W<CLOCK_CTL_SPEC> {
        DIV_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    #[must_use]
    pub fn type_sel(&mut self) -> TYPE_SEL_W<CLOCK_CTL_SPEC> {
        TYPE_SEL_W::new(self, 8)
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
#[doc = "`reset()` method sets CLOCK_CTL[%s]
to value 0x03ff"]
impl crate::Resettable for CLOCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
