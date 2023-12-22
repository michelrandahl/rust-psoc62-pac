#[doc = "Register `AMUX_SPLIT_CTL[%s]` reader"]
pub type R = crate::R<AMUX_SPLIT_CTL_SPEC>;
#[doc = "Register `AMUX_SPLIT_CTL[%s]` writer"]
pub type W = crate::W<AMUX_SPLIT_CTL_SPEC>;
#[doc = "Field `SWITCH_AA_SL` reader - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub type SWITCH_AA_SL_R = crate::BitReader;
#[doc = "Field `SWITCH_AA_SL` writer - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub type SWITCH_AA_SL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_AA_SR` reader - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub type SWITCH_AA_SR_R = crate::BitReader;
#[doc = "Field `SWITCH_AA_SR` writer - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub type SWITCH_AA_SR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_AA_S0` reader - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
pub type SWITCH_AA_S0_R = crate::BitReader;
#[doc = "Field `SWITCH_AA_S0` writer - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
pub type SWITCH_AA_S0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_BB_SL` reader - T-switch control for Left AMUXBUSB switch."]
pub type SWITCH_BB_SL_R = crate::BitReader;
#[doc = "Field `SWITCH_BB_SL` writer - T-switch control for Left AMUXBUSB switch."]
pub type SWITCH_BB_SL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_BB_SR` reader - T-switch control for Right AMUXBUSB switch."]
pub type SWITCH_BB_SR_R = crate::BitReader;
#[doc = "Field `SWITCH_BB_SR` writer - T-switch control for Right AMUXBUSB switch."]
pub type SWITCH_BB_SR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_BB_S0` reader - T-switch control for AMUXBUSB vssa/ground switch."]
pub type SWITCH_BB_S0_R = crate::BitReader;
#[doc = "Field `SWITCH_BB_S0` writer - T-switch control for AMUXBUSB vssa/ground switch."]
pub type SWITCH_BB_S0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sl(&self) -> SWITCH_AA_SL_R {
        SWITCH_AA_SL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sr(&self) -> SWITCH_AA_SR_R {
        SWITCH_AA_SR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_s0(&self) -> SWITCH_AA_S0_R {
        SWITCH_AA_S0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sl(&self) -> SWITCH_BB_SL_R {
        SWITCH_BB_SL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sr(&self) -> SWITCH_BB_SR_R {
        SWITCH_BB_SR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    pub fn switch_bb_s0(&self) -> SWITCH_BB_S0_R {
        SWITCH_BB_S0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    #[must_use]
    pub fn switch_aa_sl(&mut self) -> SWITCH_AA_SL_W<AMUX_SPLIT_CTL_SPEC> {
        SWITCH_AA_SL_W::new(self, 0)
    }
    #[doc = "Bit 1 - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    #[must_use]
    pub fn switch_aa_sr(&mut self) -> SWITCH_AA_SR_W<AMUX_SPLIT_CTL_SPEC> {
        SWITCH_AA_SR_W::new(self, 1)
    }
    #[doc = "Bit 2 - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    #[must_use]
    pub fn switch_aa_s0(&mut self) -> SWITCH_AA_S0_W<AMUX_SPLIT_CTL_SPEC> {
        SWITCH_AA_S0_W::new(self, 2)
    }
    #[doc = "Bit 4 - T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    #[must_use]
    pub fn switch_bb_sl(&mut self) -> SWITCH_BB_SL_W<AMUX_SPLIT_CTL_SPEC> {
        SWITCH_BB_SL_W::new(self, 4)
    }
    #[doc = "Bit 5 - T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    #[must_use]
    pub fn switch_bb_sr(&mut self) -> SWITCH_BB_SR_W<AMUX_SPLIT_CTL_SPEC> {
        SWITCH_BB_SR_W::new(self, 5)
    }
    #[doc = "Bit 6 - T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    #[must_use]
    pub fn switch_bb_s0(&mut self) -> SWITCH_BB_S0_W<AMUX_SPLIT_CTL_SPEC> {
        SWITCH_BB_S0_W::new(self, 6)
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
#[doc = "AMUX splitter cell control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amux_split_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amux_split_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AMUX_SPLIT_CTL_SPEC;
impl crate::RegisterSpec for AMUX_SPLIT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amux_split_ctl::R`](R) reader structure"]
impl crate::Readable for AMUX_SPLIT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`amux_split_ctl::W`](W) writer structure"]
impl crate::Writable for AMUX_SPLIT_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMUX_SPLIT_CTL[%s]
to value 0"]
impl crate::Resettable for AMUX_SPLIT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
