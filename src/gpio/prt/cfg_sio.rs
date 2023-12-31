#[doc = "Register `CFG_SIO` reader"]
pub type R = crate::R<CFG_SIO_SPEC>;
#[doc = "Register `CFG_SIO` writer"]
pub type W = crate::W<CFG_SIO_SPEC>;
#[doc = "Field `VREG_EN01` reader - The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
pub type VREG_EN01_R = crate::BitReader;
#[doc = "Field `VREG_EN01` writer - The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
pub type VREG_EN01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUF_SEL01` reader - N/A"]
pub type IBUF_SEL01_R = crate::BitReader;
#[doc = "Field `IBUF_SEL01` writer - N/A"]
pub type IBUF_SEL01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL01` reader - N/A"]
pub type VTRIP_SEL01_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL01` writer - N/A"]
pub type VTRIP_SEL01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_SEL01` reader - N/A"]
pub type VREF_SEL01_R = crate::FieldReader;
#[doc = "Field `VREF_SEL01` writer - N/A"]
pub type VREF_SEL01_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VOH_SEL01` reader - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1'): VTRIP_SEL=0: a) VREF_SEL=00, VOH_SEL=X -> Trip point=50 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=Vohref (buffered) c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. VTRIP_SEL=1: a) VREF_SEL=00, VOH_SEL=X -> Trip point=40 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=0.5*Vohref c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=0.5*Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer."]
pub type VOH_SEL01_R = crate::FieldReader;
#[doc = "Field `VOH_SEL01` writer - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1'): VTRIP_SEL=0: a) VREF_SEL=00, VOH_SEL=X -> Trip point=50 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=Vohref (buffered) c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. VTRIP_SEL=1: a) VREF_SEL=00, VOH_SEL=X -> Trip point=40 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=0.5*Vohref c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=0.5*Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer."]
pub type VOH_SEL01_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VREG_EN23` reader - N/A"]
pub type VREG_EN23_R = crate::BitReader;
#[doc = "Field `VREG_EN23` writer - N/A"]
pub type VREG_EN23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUF_SEL23` reader - N/A"]
pub type IBUF_SEL23_R = crate::BitReader;
#[doc = "Field `IBUF_SEL23` writer - N/A"]
pub type IBUF_SEL23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL23` reader - N/A"]
pub type VTRIP_SEL23_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL23` writer - N/A"]
pub type VTRIP_SEL23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_SEL23` reader - N/A"]
pub type VREF_SEL23_R = crate::FieldReader;
#[doc = "Field `VREF_SEL23` writer - N/A"]
pub type VREF_SEL23_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VOH_SEL23` reader - N/A"]
pub type VOH_SEL23_R = crate::FieldReader;
#[doc = "Field `VOH_SEL23` writer - N/A"]
pub type VOH_SEL23_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VREG_EN45` reader - N/A"]
pub type VREG_EN45_R = crate::BitReader;
#[doc = "Field `VREG_EN45` writer - N/A"]
pub type VREG_EN45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUF_SEL45` reader - N/A"]
pub type IBUF_SEL45_R = crate::BitReader;
#[doc = "Field `IBUF_SEL45` writer - N/A"]
pub type IBUF_SEL45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL45` reader - N/A"]
pub type VTRIP_SEL45_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL45` writer - N/A"]
pub type VTRIP_SEL45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_SEL45` reader - N/A"]
pub type VREF_SEL45_R = crate::FieldReader;
#[doc = "Field `VREF_SEL45` writer - N/A"]
pub type VREF_SEL45_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VOH_SEL45` reader - N/A"]
pub type VOH_SEL45_R = crate::FieldReader;
#[doc = "Field `VOH_SEL45` writer - N/A"]
pub type VOH_SEL45_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VREG_EN67` reader - N/A"]
pub type VREG_EN67_R = crate::BitReader;
#[doc = "Field `VREG_EN67` writer - N/A"]
pub type VREG_EN67_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUF_SEL67` reader - N/A"]
pub type IBUF_SEL67_R = crate::BitReader;
#[doc = "Field `IBUF_SEL67` writer - N/A"]
pub type IBUF_SEL67_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL67` reader - N/A"]
pub type VTRIP_SEL67_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL67` writer - N/A"]
pub type VTRIP_SEL67_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_SEL67` reader - N/A"]
pub type VREF_SEL67_R = crate::FieldReader;
#[doc = "Field `VREF_SEL67` writer - N/A"]
pub type VREF_SEL67_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VOH_SEL67` reader - N/A"]
pub type VOH_SEL67_R = crate::FieldReader;
#[doc = "Field `VOH_SEL67` writer - N/A"]
pub type VOH_SEL67_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub fn vreg_en01(&self) -> VREG_EN01_R {
        VREG_EN01_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel01(&self) -> IBUF_SEL01_R {
        IBUF_SEL01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel01(&self) -> VTRIP_SEL01_R {
        VTRIP_SEL01_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - N/A"]
    #[inline(always)]
    pub fn vref_sel01(&self) -> VREF_SEL01_R {
        VREF_SEL01_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1'): VTRIP_SEL=0: a) VREF_SEL=00, VOH_SEL=X -> Trip point=50 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=Vohref (buffered) c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. VTRIP_SEL=1: a) VREF_SEL=00, VOH_SEL=X -> Trip point=40 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=0.5*Vohref c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=0.5*Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer."]
    #[inline(always)]
    pub fn voh_sel01(&self) -> VOH_SEL01_R {
        VOH_SEL01_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn vreg_en23(&self) -> VREG_EN23_R {
        VREG_EN23_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel23(&self) -> IBUF_SEL23_R {
        IBUF_SEL23_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel23(&self) -> VTRIP_SEL23_R {
        VTRIP_SEL23_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - N/A"]
    #[inline(always)]
    pub fn vref_sel23(&self) -> VREF_SEL23_R {
        VREF_SEL23_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - N/A"]
    #[inline(always)]
    pub fn voh_sel23(&self) -> VOH_SEL23_R {
        VOH_SEL23_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn vreg_en45(&self) -> VREG_EN45_R {
        VREG_EN45_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel45(&self) -> IBUF_SEL45_R {
        IBUF_SEL45_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel45(&self) -> VTRIP_SEL45_R {
        VTRIP_SEL45_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - N/A"]
    #[inline(always)]
    pub fn vref_sel45(&self) -> VREF_SEL45_R {
        VREF_SEL45_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - N/A"]
    #[inline(always)]
    pub fn voh_sel45(&self) -> VOH_SEL45_R {
        VOH_SEL45_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn vreg_en67(&self) -> VREG_EN67_R {
        VREG_EN67_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel67(&self) -> IBUF_SEL67_R {
        IBUF_SEL67_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel67(&self) -> VTRIP_SEL67_R {
        VTRIP_SEL67_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - N/A"]
    #[inline(always)]
    pub fn vref_sel67(&self) -> VREF_SEL67_R {
        VREF_SEL67_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:31 - N/A"]
    #[inline(always)]
    pub fn voh_sel67(&self) -> VOH_SEL67_R {
        VOH_SEL67_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en01(&mut self) -> VREG_EN01_W<CFG_SIO_SPEC> {
        VREG_EN01_W::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel01(&mut self) -> IBUF_SEL01_W<CFG_SIO_SPEC> {
        IBUF_SEL01_W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel01(&mut self) -> VTRIP_SEL01_W<CFG_SIO_SPEC> {
        VTRIP_SEL01_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel01(&mut self) -> VREF_SEL01_W<CFG_SIO_SPEC> {
        VREF_SEL01_W::new(self, 3)
    }
    #[doc = "Bits 5:7 - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1'): VTRIP_SEL=0: a) VREF_SEL=00, VOH_SEL=X -> Trip point=50 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=Vohref (buffered) c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. VTRIP_SEL=1: a) VREF_SEL=00, VOH_SEL=X -> Trip point=40 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=0.5*Vohref c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=0.5*Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer."]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel01(&mut self) -> VOH_SEL01_W<CFG_SIO_SPEC> {
        VOH_SEL01_W::new(self, 5)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en23(&mut self) -> VREG_EN23_W<CFG_SIO_SPEC> {
        VREG_EN23_W::new(self, 8)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel23(&mut self) -> IBUF_SEL23_W<CFG_SIO_SPEC> {
        IBUF_SEL23_W::new(self, 9)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel23(&mut self) -> VTRIP_SEL23_W<CFG_SIO_SPEC> {
        VTRIP_SEL23_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel23(&mut self) -> VREF_SEL23_W<CFG_SIO_SPEC> {
        VREF_SEL23_W::new(self, 11)
    }
    #[doc = "Bits 13:15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel23(&mut self) -> VOH_SEL23_W<CFG_SIO_SPEC> {
        VOH_SEL23_W::new(self, 13)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en45(&mut self) -> VREG_EN45_W<CFG_SIO_SPEC> {
        VREG_EN45_W::new(self, 16)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel45(&mut self) -> IBUF_SEL45_W<CFG_SIO_SPEC> {
        IBUF_SEL45_W::new(self, 17)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel45(&mut self) -> VTRIP_SEL45_W<CFG_SIO_SPEC> {
        VTRIP_SEL45_W::new(self, 18)
    }
    #[doc = "Bits 19:20 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel45(&mut self) -> VREF_SEL45_W<CFG_SIO_SPEC> {
        VREF_SEL45_W::new(self, 19)
    }
    #[doc = "Bits 21:23 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel45(&mut self) -> VOH_SEL45_W<CFG_SIO_SPEC> {
        VOH_SEL45_W::new(self, 21)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en67(&mut self) -> VREG_EN67_W<CFG_SIO_SPEC> {
        VREG_EN67_W::new(self, 24)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel67(&mut self) -> IBUF_SEL67_W<CFG_SIO_SPEC> {
        IBUF_SEL67_W::new(self, 25)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel67(&mut self) -> VTRIP_SEL67_W<CFG_SIO_SPEC> {
        VTRIP_SEL67_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel67(&mut self) -> VREF_SEL67_W<CFG_SIO_SPEC> {
        VREF_SEL67_W::new(self, 27)
    }
    #[doc = "Bits 29:31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel67(&mut self) -> VOH_SEL67_W<CFG_SIO_SPEC> {
        VOH_SEL67_W::new(self, 29)
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
#[doc = "Port SIO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_sio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_sio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SIO_SPEC;
impl crate::RegisterSpec for CFG_SIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_sio::R`](R) reader structure"]
impl crate::Readable for CFG_SIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_sio::W`](W) writer structure"]
impl crate::Writable for CFG_SIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_SIO to value 0"]
impl crate::Resettable for CFG_SIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
