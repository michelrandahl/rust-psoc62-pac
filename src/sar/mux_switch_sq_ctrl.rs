#[doc = "Register `MUX_SWITCH_SQ_CTRL` reader"]
pub type R = crate::R<MUX_SWITCH_SQ_CTRL_SPEC>;
#[doc = "Register `MUX_SWITCH_SQ_CTRL` writer"]
pub type W = crate::W<MUX_SWITCH_SQ_CTRL_SPEC>;
#[doc = "Field `MUX_SQ_CTRL_P0` reader - for P0 switches"]
pub type MUX_SQ_CTRL_P0_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P0` writer - for P0 switches"]
pub type MUX_SQ_CTRL_P0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P1` reader - for P1 switches"]
pub type MUX_SQ_CTRL_P1_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P1` writer - for P1 switches"]
pub type MUX_SQ_CTRL_P1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P2` reader - for P2 switches"]
pub type MUX_SQ_CTRL_P2_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P2` writer - for P2 switches"]
pub type MUX_SQ_CTRL_P2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P3` reader - for P3 switches"]
pub type MUX_SQ_CTRL_P3_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P3` writer - for P3 switches"]
pub type MUX_SQ_CTRL_P3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P4` reader - for P4 switches"]
pub type MUX_SQ_CTRL_P4_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P4` writer - for P4 switches"]
pub type MUX_SQ_CTRL_P4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P5` reader - for P5 switches"]
pub type MUX_SQ_CTRL_P5_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P5` writer - for P5 switches"]
pub type MUX_SQ_CTRL_P5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P6` reader - for P6 switches"]
pub type MUX_SQ_CTRL_P6_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P6` writer - for P6 switches"]
pub type MUX_SQ_CTRL_P6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_P7` reader - for P7 switches"]
pub type MUX_SQ_CTRL_P7_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_P7` writer - for P7 switches"]
pub type MUX_SQ_CTRL_P7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_VSSA` reader - for vssa switch"]
pub type MUX_SQ_CTRL_VSSA_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_VSSA` writer - for vssa switch"]
pub type MUX_SQ_CTRL_VSSA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_TEMP` reader - for temp switch"]
pub type MUX_SQ_CTRL_TEMP_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_TEMP` writer - for temp switch"]
pub type MUX_SQ_CTRL_TEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSA` reader - for amuxbusa switch"]
pub type MUX_SQ_CTRL_AMUXBUSA_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSA` writer - for amuxbusa switch"]
pub type MUX_SQ_CTRL_AMUXBUSA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSB` reader - for amuxbusb switches"]
pub type MUX_SQ_CTRL_AMUXBUSB_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_AMUXBUSB` writer - for amuxbusb switches"]
pub type MUX_SQ_CTRL_AMUXBUSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_SARBUS0` reader - for sarbus0 switch"]
pub type MUX_SQ_CTRL_SARBUS0_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_SARBUS0` writer - for sarbus0 switch"]
pub type MUX_SQ_CTRL_SARBUS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SQ_CTRL_SARBUS1` reader - for sarbus1 switch"]
pub type MUX_SQ_CTRL_SARBUS1_R = crate::BitReader;
#[doc = "Field `MUX_SQ_CTRL_SARBUS1` writer - for sarbus1 switch"]
pub type MUX_SQ_CTRL_SARBUS1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - for P0 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p0(&self) -> MUX_SQ_CTRL_P0_R {
        MUX_SQ_CTRL_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - for P1 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p1(&self) -> MUX_SQ_CTRL_P1_R {
        MUX_SQ_CTRL_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - for P2 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p2(&self) -> MUX_SQ_CTRL_P2_R {
        MUX_SQ_CTRL_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - for P3 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p3(&self) -> MUX_SQ_CTRL_P3_R {
        MUX_SQ_CTRL_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - for P4 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p4(&self) -> MUX_SQ_CTRL_P4_R {
        MUX_SQ_CTRL_P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - for P5 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p5(&self) -> MUX_SQ_CTRL_P5_R {
        MUX_SQ_CTRL_P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - for P6 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p6(&self) -> MUX_SQ_CTRL_P6_R {
        MUX_SQ_CTRL_P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - for P7 switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_p7(&self) -> MUX_SQ_CTRL_P7_R {
        MUX_SQ_CTRL_P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - for vssa switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_vssa(&self) -> MUX_SQ_CTRL_VSSA_R {
        MUX_SQ_CTRL_VSSA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - for temp switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_temp(&self) -> MUX_SQ_CTRL_TEMP_R {
        MUX_SQ_CTRL_TEMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - for amuxbusa switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_amuxbusa(&self) -> MUX_SQ_CTRL_AMUXBUSA_R {
        MUX_SQ_CTRL_AMUXBUSA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - for amuxbusb switches"]
    #[inline(always)]
    pub fn mux_sq_ctrl_amuxbusb(&self) -> MUX_SQ_CTRL_AMUXBUSB_R {
        MUX_SQ_CTRL_AMUXBUSB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - for sarbus0 switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_sarbus0(&self) -> MUX_SQ_CTRL_SARBUS0_R {
        MUX_SQ_CTRL_SARBUS0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - for sarbus1 switch"]
    #[inline(always)]
    pub fn mux_sq_ctrl_sarbus1(&self) -> MUX_SQ_CTRL_SARBUS1_R {
        MUX_SQ_CTRL_SARBUS1_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - for P0 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p0(&mut self) -> MUX_SQ_CTRL_P0_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_P0_W::new(self, 0)
    }
    #[doc = "Bit 1 - for P1 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p1(&mut self) -> MUX_SQ_CTRL_P1_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_P1_W::new(self, 1)
    }
    #[doc = "Bit 2 - for P2 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p2(&mut self) -> MUX_SQ_CTRL_P2_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_P2_W::new(self, 2)
    }
    #[doc = "Bit 3 - for P3 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p3(&mut self) -> MUX_SQ_CTRL_P3_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_P3_W::new(self, 3)
    }
    #[doc = "Bit 4 - for P4 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p4(&mut self) -> MUX_SQ_CTRL_P4_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_P4_W::new(self, 4)
    }
    #[doc = "Bit 5 - for P5 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p5(&mut self) -> MUX_SQ_CTRL_P5_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_P5_W::new(self, 5)
    }
    #[doc = "Bit 6 - for P6 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p6(&mut self) -> MUX_SQ_CTRL_P6_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_P6_W::new(self, 6)
    }
    #[doc = "Bit 7 - for P7 switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_p7(&mut self) -> MUX_SQ_CTRL_P7_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_P7_W::new(self, 7)
    }
    #[doc = "Bit 16 - for vssa switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_vssa(&mut self) -> MUX_SQ_CTRL_VSSA_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_VSSA_W::new(self, 16)
    }
    #[doc = "Bit 17 - for temp switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_temp(&mut self) -> MUX_SQ_CTRL_TEMP_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_TEMP_W::new(self, 17)
    }
    #[doc = "Bit 18 - for amuxbusa switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_amuxbusa(&mut self) -> MUX_SQ_CTRL_AMUXBUSA_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_AMUXBUSA_W::new(self, 18)
    }
    #[doc = "Bit 19 - for amuxbusb switches"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_amuxbusb(&mut self) -> MUX_SQ_CTRL_AMUXBUSB_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_AMUXBUSB_W::new(self, 19)
    }
    #[doc = "Bit 22 - for sarbus0 switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_sarbus0(&mut self) -> MUX_SQ_CTRL_SARBUS0_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_SARBUS0_W::new(self, 22)
    }
    #[doc = "Bit 23 - for sarbus1 switch"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sq_ctrl_sarbus1(&mut self) -> MUX_SQ_CTRL_SARBUS1_W<MUX_SWITCH_SQ_CTRL_SPEC> {
        MUX_SQ_CTRL_SARBUS1_W::new(self, 23)
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
#[doc = "SARMUX switch Sar Sequencer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux_switch_sq_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux_switch_sq_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUX_SWITCH_SQ_CTRL_SPEC;
impl crate::RegisterSpec for MUX_SWITCH_SQ_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mux_switch_sq_ctrl::R`](R) reader structure"]
impl crate::Readable for MUX_SWITCH_SQ_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mux_switch_sq_ctrl::W`](W) writer structure"]
impl crate::Writable for MUX_SWITCH_SQ_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUX_SWITCH_SQ_CTRL to value 0"]
impl crate::Resettable for MUX_SWITCH_SQ_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
