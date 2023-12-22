#[doc = "Register `RAM1_CTL0` reader"]
pub type R = crate::R<RAM1_CTL0_SPEC>;
#[doc = "Register `RAM1_CTL0` writer"]
pub type W = crate::W<RAM1_CTL0_SPEC>;
#[doc = "Field `SLOW_WS` reader - See RAM0_CTL."]
pub type SLOW_WS_R = crate::FieldReader;
#[doc = "Field `SLOW_WS` writer - See RAM0_CTL."]
pub type SLOW_WS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FAST_WS` reader - See RAM0_CTL."]
pub type FAST_WS_R = crate::FieldReader;
#[doc = "Field `FAST_WS` writer - See RAM0_CTL."]
pub type FAST_WS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_EN` reader - See RAM0_CTL."]
pub type ECC_EN_R = crate::BitReader;
#[doc = "Field `ECC_EN` writer - See RAM0_CTL."]
pub type ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_AUTO_CORRECT` reader - See RAM0_CTL."]
pub type ECC_AUTO_CORRECT_R = crate::BitReader;
#[doc = "Field `ECC_AUTO_CORRECT` writer - See RAM0_CTL."]
pub type ECC_AUTO_CORRECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_INJ_EN` reader - See RAM0_CTL."]
pub type ECC_INJ_EN_R = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - See RAM0_CTL."]
pub type ECC_INJ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - See RAM0_CTL."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SLOW_WS_R {
        SLOW_WS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - See RAM0_CTL."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FAST_WS_R {
        FAST_WS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - See RAM0_CTL."]
    #[inline(always)]
    pub fn ecc_en(&self) -> ECC_EN_R {
        ECC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See RAM0_CTL."]
    #[inline(always)]
    pub fn ecc_auto_correct(&self) -> ECC_AUTO_CORRECT_R {
        ECC_AUTO_CORRECT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - See RAM0_CTL."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> ECC_INJ_EN_R {
        ECC_INJ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn slow_ws(&mut self) -> SLOW_WS_W<RAM1_CTL0_SPEC> {
        SLOW_WS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn fast_ws(&mut self) -> FAST_WS_W<RAM1_CTL0_SPEC> {
        FAST_WS_W::new(self, 8)
    }
    #[doc = "Bit 16 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> ECC_EN_W<RAM1_CTL0_SPEC> {
        ECC_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_auto_correct(&mut self) -> ECC_AUTO_CORRECT_W<RAM1_CTL0_SPEC> {
        ECC_AUTO_CORRECT_W::new(self, 17)
    }
    #[doc = "Bit 18 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> ECC_INJ_EN_W<RAM1_CTL0_SPEC> {
        ECC_INJ_EN_W::new(self, 18)
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
#[doc = "RAM 1 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram1_ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram1_ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM1_CTL0_SPEC;
impl crate::RegisterSpec for RAM1_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram1_ctl0::R`](R) reader structure"]
impl crate::Readable for RAM1_CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ram1_ctl0::W`](W) writer structure"]
impl crate::Writable for RAM1_CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAM1_CTL0 to value 0x0003_0001"]
impl crate::Resettable for RAM1_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0001;
}
