#[doc = "Register `ROM_CTL` reader"]
pub type R = crate::R<ROM_CTL_SPEC>;
#[doc = "Register `ROM_CTL` writer"]
pub type W = crate::W<ROM_CTL_SPEC>;
#[doc = "Field `SLOW_WS` reader - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. A table/formula will be provided for this field's values for different 'clk_hf' frequencies."]
pub type SLOW_WS_R = crate::FieldReader;
#[doc = "Field `SLOW_WS` writer - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. A table/formula will be provided for this field's values for different 'clk_hf' frequencies."]
pub type SLOW_WS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FAST_WS` reader - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type FAST_WS_R = crate::FieldReader;
#[doc = "Field `FAST_WS` writer - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type FAST_WS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. A table/formula will be provided for this field's values for different 'clk_hf' frequencies."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SLOW_WS_R {
        SLOW_WS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FAST_WS_R {
        FAST_WS_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. A table/formula will be provided for this field's values for different 'clk_hf' frequencies."]
    #[inline(always)]
    #[must_use]
    pub fn slow_ws(&mut self) -> SLOW_WS_W<ROM_CTL_SPEC> {
        SLOW_WS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    #[must_use]
    pub fn fast_ws(&mut self) -> FAST_WS_W<ROM_CTL_SPEC> {
        FAST_WS_W::new(self, 8)
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
#[doc = "ROM control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_CTL_SPEC;
impl crate::RegisterSpec for ROM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_ctl::R`](R) reader structure"]
impl crate::Readable for ROM_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_ctl::W`](W) writer structure"]
impl crate::Writable for ROM_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROM_CTL to value 0x01"]
impl crate::Resettable for ROM_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
