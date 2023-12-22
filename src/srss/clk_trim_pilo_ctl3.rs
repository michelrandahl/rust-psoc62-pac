#[doc = "Register `CLK_TRIM_PILO_CTL3` reader"]
pub type R = crate::R<CLK_TRIM_PILO_CTL3_SPEC>;
#[doc = "Register `CLK_TRIM_PILO_CTL3` writer"]
pub type W = crate::W<CLK_TRIM_PILO_CTL3_SPEC>;
#[doc = "Field `PILO_ENGOPT` reader - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
pub type PILO_ENGOPT_R = crate::FieldReader<u16>;
#[doc = "Field `PILO_ENGOPT` writer - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
pub type PILO_ENGOPT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
    #[inline(always)]
    pub fn pilo_engopt(&self) -> PILO_ENGOPT_R {
        PILO_ENGOPT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_engopt(&mut self) -> PILO_ENGOPT_W<CLK_TRIM_PILO_CTL3_SPEC> {
        PILO_ENGOPT_W::new(self, 0)
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
#[doc = "PILO Trim Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_pilo_ctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_TRIM_PILO_CTL3_SPEC;
impl crate::RegisterSpec for CLK_TRIM_PILO_CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_pilo_ctl3::R`](R) reader structure"]
impl crate::Readable for CLK_TRIM_PILO_CTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_pilo_ctl3::W`](W) writer structure"]
impl crate::Writable for CLK_TRIM_PILO_CTL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_PILO_CTL3 to value 0x4800"]
impl crate::Resettable for CLK_TRIM_PILO_CTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x4800;
}
