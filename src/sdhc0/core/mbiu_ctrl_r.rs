#[doc = "Register `MBIU_CTRL_R` reader"]
pub type R = crate::R<MBIU_CTRL_R_SPEC>;
#[doc = "Register `MBIU_CTRL_R` writer"]
pub type W = crate::W<MBIU_CTRL_R_SPEC>;
#[doc = "Field `UNDEFL_INCR_EN` reader - Undefined INCR Burst Controls generation of undefined length INCR transfer on Master interface. Values: - 0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F - 0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
pub type UNDEFL_INCR_EN_R = crate::BitReader;
#[doc = "Field `UNDEFL_INCR_EN` writer - Undefined INCR Burst Controls generation of undefined length INCR transfer on Master interface. Values: - 0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F - 0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
pub type UNDEFL_INCR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURST_INCR4_EN` reader - INCR4 Burst Controls generation of INCR4 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
pub type BURST_INCR4_EN_R = crate::BitReader;
#[doc = "Field `BURST_INCR4_EN` writer - INCR4 Burst Controls generation of INCR4 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
pub type BURST_INCR4_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURST_INCR8_EN` reader - INCR8 Burst Controls generation of INCR8 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
pub type BURST_INCR8_EN_R = crate::BitReader;
#[doc = "Field `BURST_INCR8_EN` writer - INCR8 Burst Controls generation of INCR8 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
pub type BURST_INCR8_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURST_INCR16_EN` reader - INCR16 Burst Controls generation of INCR16 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
pub type BURST_INCR16_EN_R = crate::BitReader;
#[doc = "Field `BURST_INCR16_EN` writer - INCR16 Burst Controls generation of INCR16 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
pub type BURST_INCR16_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Undefined INCR Burst Controls generation of undefined length INCR transfer on Master interface. Values: - 0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F - 0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
    #[inline(always)]
    pub fn undefl_incr_en(&self) -> UNDEFL_INCR_EN_R {
        UNDEFL_INCR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INCR4 Burst Controls generation of INCR4 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr4_en(&self) -> BURST_INCR4_EN_R {
        BURST_INCR4_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INCR8 Burst Controls generation of INCR8 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr8_en(&self) -> BURST_INCR8_EN_R {
        BURST_INCR8_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - INCR16 Burst Controls generation of INCR16 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr16_en(&self) -> BURST_INCR16_EN_R {
        BURST_INCR16_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Undefined INCR Burst Controls generation of undefined length INCR transfer on Master interface. Values: - 0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F - 0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
    #[inline(always)]
    #[must_use]
    pub fn undefl_incr_en(&mut self) -> UNDEFL_INCR_EN_W<MBIU_CTRL_R_SPEC> {
        UNDEFL_INCR_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - INCR4 Burst Controls generation of INCR4 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
    #[inline(always)]
    #[must_use]
    pub fn burst_incr4_en(&mut self) -> BURST_INCR4_EN_W<MBIU_CTRL_R_SPEC> {
        BURST_INCR4_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - INCR8 Burst Controls generation of INCR8 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
    #[inline(always)]
    #[must_use]
    pub fn burst_incr8_en(&mut self) -> BURST_INCR8_EN_W<MBIU_CTRL_R_SPEC> {
        BURST_INCR8_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - INCR16 Burst Controls generation of INCR16 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
    #[inline(always)]
    #[must_use]
    pub fn burst_incr16_en(&mut self) -> BURST_INCR16_EN_W<MBIU_CTRL_R_SPEC> {
        BURST_INCR16_EN_W::new(self, 3)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MBIU Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbiu_ctrl_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbiu_ctrl_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MBIU_CTRL_R_SPEC;
impl crate::RegisterSpec for MBIU_CTRL_R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mbiu_ctrl_r::R`](R) reader structure"]
impl crate::Readable for MBIU_CTRL_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mbiu_ctrl_r::W`](W) writer structure"]
impl crate::Writable for MBIU_CTRL_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MBIU_CTRL_R to value 0x01"]
impl crate::Resettable for MBIU_CTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
